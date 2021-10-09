use crate::documents::NoticeDocument;
use crate::error::Result;
use async_graphql::InputObject;
use chrono::Utc;
use piteo_core::activity::trace;
use piteo_core::database::Db;
use piteo_core::pdfmaker::Pdfmaker;
use piteo_data::notice_filename;
use piteo_data::AuthId;
use piteo_data::DateTime;
use piteo_data::EventType;
use piteo_data::FileType;
use piteo_data::PaymentNotice;
use piteo_data::PaymentNoticeId;
use piteo_data::Rent;
use piteo_data::RentData;
use piteo_data::RentId;
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
) -> Result<Vec<PaymentNotice>> {
    input.validate()?;

    let rents = find_rents(db, auth_id, input.rent_ids)?;

    let notices = generate_notices(db, auth_id, pdfmaker, rents).await?;

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
    auth_id: &AuthId,
    pdfmaker: &impl Pdfmaker,
    rents: Vec<Rent>,
) -> Result<Vec<PaymentNotice>> {
    let mut notices = vec![];

    for rent in rents {
        // Try to fetch associated entities.
        let lease = db.leases().by_id(&rent.lease_id)?;
        let tenants = db.tenants().by_lease_id(&lease.id)?;
        let property = db.properties().by_id(&lease.property_id)?;
        let lender = db.lenders().by_id(&property.lender_id)?;

        // Init new notice.
        let notice_id = PaymentNoticeId::new();
        let mut notice = PaymentNotice {
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

        notices.push(notice);

        trace(db, auth_id, EventType::PaymentNoticeCreated, rent.id).ok();
    }

    Ok(notices)
}
