use crate::error::Result;
use async_graphql::InputObject;
use chrono::Utc;
use trankeel_core::context::Context;
use trankeel_core::database::Db;
use trankeel_core::dispatcher::dispatch;
use trankeel_core::dispatcher::Event;
use trankeel_core::pdfmaker::Pdfmaker;
use trankeel_core::templates::NoticeDocument;
use trankeel_data::notice_filename;
use trankeel_data::AuthId;
use trankeel_data::DateTime;
use trankeel_data::FileType;
use trankeel_data::Notice;
use trankeel_data::NoticeId;
use trankeel_data::Rent;
use trankeel_data::RentId;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
#[graphql(name = "SendPaymentNoticeInput")]
pub struct CreateNoticesInput {
    rent_ids: Vec<RentId>,
    date: Option<DateTime>,
}

// # Operation

pub async fn create_notices(
    ctx: &Context,
    auth_id: &AuthId,
    input: CreateNoticesInput,
) -> Result<Vec<Notice>> {
    input.validate()?;

    let rents = find_rents(ctx, auth_id, input.rent_ids)?;

    let notices = generate_notices(ctx, rents).await?;

    Ok(notices)
}

// # Utils

fn find_rents(ctx: &Context, _auth_id: &AuthId, rent_ids: Vec<RentId>) -> Result<Vec<Rent>> {
    let db = ctx.db();

    let mut rents = vec![];

    for rent_id in rent_ids {
        let rent = db.rents().by_id(&rent_id)?;
        rents.push(rent);
    }

    Ok(rents)
}

async fn generate_notices(ctx: &Context, rents: Vec<Rent>) -> Result<Vec<Notice>> {
    let db = ctx.db();
    let pdfmaker = ctx.pdfmaker();

    let mut notices = vec![];

    for rent in rents {
        // Try to fetch associated entities.
        let lease = db.leases().by_id(&rent.lease_id)?;
        let tenants = db.tenants().by_lease_id(&lease.id)?;
        let property = db.properties().by_id(&lease.property_id)?;
        let lender = db.lenders().by_id(&property.lender_id)?;

        // Init new notice.
        let notice_id = NoticeId::new();
        let mut notice = Notice {
            id: notice_id,
            type_: FileType::PaymentNotice,
            filename: Some(notice_filename(&notice_id, &rent)),
            status: None,
            external_id: None,
            download_url: None,
            preview_url: None,
            created_at: None,
            updated_at: None,
        };

        // Try to generate notice document (PDF).
        let document = NoticeDocument::try_new(
            notice.clone(),
            rent.clone(),
            lender,
            tenants,
            property,
            Utc::now().into(),
        )?;
        let document = pdfmaker.generate(document).await?;

        // Assign notice external ID.
        notice.external_id = Some(document.id);
        notice.status = Some(document.status);

        // Create notice.
        let notice = match db.files().create(&notice) {
            Ok(notice) => notice,
            Err(err) => return Err(err),
        };

        // Link notice with rent.
        db.rents().update(&Rent {
            id: rent.id,
            notice_id: Some(notice.id),
            ..rent
        })?;

        notices.push(notice.clone());

        dispatch(ctx, vec![Event::NoticeCreated(notice)])?;
    }

    Ok(notices)
}
