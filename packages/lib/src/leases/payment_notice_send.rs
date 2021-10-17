use crate::error::Result;
use crate::templates::NoticeDocument;
use async_graphql::InputObject;
use chrono::Utc;
use trankeel_core::activity::trace;
use trankeel_core::activity::Trace;
use trankeel_core::database::Db;
use trankeel_core::pdfmaker::Pdfmaker;
use trankeel_data::notice_filename;
use trankeel_data::AuthId;
use trankeel_data::DateTime;
use trankeel_data::FileType;
use trankeel_data::Notice;
use trankeel_data::NoticeId;
use trankeel_data::Rent;
use trankeel_data::RentData;
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
    db: &impl Db,
    auth_id: &AuthId,
    pdfmaker: &impl Pdfmaker,
    input: CreateNoticesInput,
) -> Result<Vec<Notice>> {
    input.validate()?;

    let rents = find_rents(db, auth_id, input.rent_ids)?;

    let notices = generate_notices(db, pdfmaker, rents).await?;

    Ok(notices)
}

// # Utils

fn find_rents(db: &impl Db, _auth_id: &AuthId, rent_ids: Vec<RentId>) -> Result<Vec<Rent>> {
    let mut rents = vec![];

    for rent_id in rent_ids {
        let rent = db.rents().by_id(&rent_id)?;
        rents.push(rent);
    }

    Ok(rents)
}

async fn generate_notices(
    db: &impl Db,
    pdfmaker: &impl Pdfmaker,
    rents: Vec<Rent>,
) -> Result<Vec<Notice>> {
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
        let notice = match db.files().create(notice) {
            Ok(notice) => notice,
            Err(err) => return Err(err),
        };

        // Link notice with rent.
        db.rents().update(RentData {
            id: rent.id,
            notice_id: Some(notice.id),
            ..Default::default()
        })?;

        notices.push(notice.clone());

        trace(db, Trace::NoticeCreated(notice)).ok();
    }

    Ok(notices)
}
