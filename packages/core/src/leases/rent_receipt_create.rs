use crate::database::Db;
use crate::documents::ReceiptDocument;
use crate::mailer::Mailer;
use crate::messages::ReceiptMail;
use crate::pdfmaker::Pdfmaker;
use async_graphql::InputObject;
use chrono::Utc;
use diesel::result::Error::NotFound;
use eyre::Error;
use piteo_data::Attachable;
use piteo_data::FileType;
use piteo_data::Receipt;
use piteo_data::ReceiptId;
use piteo_data::Rent;
use piteo_data::RentData;
use piteo_data::RentId;
use piteo_data::RentStatus;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
#[graphql(name = "RentReceiptInput")]
pub struct CreateReceiptsInput {
    rent_ids: Vec<RentId>,
    send_mail: bool,
}

#[derive(InputObject, Validate)]
pub struct SendReceiptsInput {
    pub rent_ids: Vec<RentId>,
}

// # Operation

pub fn create_receipts(
    db: &impl Db,
    pdfmaker: &impl Pdfmaker,
    input: CreateReceiptsInput,
) -> Result<Vec<Receipt>, Error> {
    input.validate()?;

    let rents = setlle_rents(db, input.rent_ids)?;

    let receipts = generate_receipts(db, pdfmaker, rents)?;

    Ok(receipts)
}

pub fn send_receipts(
    db: &impl Db,
    mailer: &impl Mailer,
    input: SendReceiptsInput,
) -> Result<Vec<Receipt>, Error> {
    input.validate()?;

    let receipts = input
        .rent_ids
        .iter()
        .map(|rent_id| {
            let rent = db.rents().by_id(rent_id)?;
            let lease = db.leases().by_id(rent.lease_id)?;
            let tenants = db.tenants().by_lease_id(lease.id)?;

            let receipt_id = rent.receipt_id.ok_or_else(|| Error::new(NotFound))?;
            let receipt = match db.files().by_id(receipt_id) {
                Ok(receipt) => receipt,
                Err(err) => return Err(err),
            };

            let mail = ReceiptMail::try_new(receipt.clone(), rent, tenants, Utc::now().into())?;
            mailer.batch(vec![mail])?;

            Ok(receipt)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(receipts)
}

// # Utils

fn setlle_rents(db: &impl Db, rent_ids: Vec<RentId>) -> Result<Vec<Rent>, Error> {
    let rents = rent_ids
        .iter()
        .map(|&rent_id| RentData {
            id: rent_id,
            status: Some(RentStatus::Settled),
            ..Default::default()
        })
        .collect();

    db.rents().update_many(rents)
}

fn generate_receipts(
    db: &impl Db,
    pdfmaker: &impl Pdfmaker,
    rents: Vec<Rent>,
) -> Result<Vec<Receipt>, Error> {
    let receipts = rents
        .iter()
        .map(|rent| {
            // Try to fetch associated entities.
            let lease = db.leases().by_id(rent.lease_id)?;
            let tenants = db.tenants().by_lease_id(lease.id)?;
            let property = db.properties().by_id(lease.property_id)?;
            let lender = db.lenders().by_id(property.lender_id)?;

            // Init new receipt.
            let receipt_id = ReceiptId::new_v4();
            let mut receipt = Receipt {
                id: receipt_id,
                type_: FileType::RentReceipt,
                filename: Some(rent.to_filename(&receipt_id)),
                status: None,
                external_id: None,
                download_url: None,
                preview_url: None,
                created_at: None,
                updated_at: None,
            };

            // Try to generate receipt document (PDF).
            let document = ReceiptDocument::try_new(
                receipt.clone(),
                rent.clone(),
                lender,
                tenants,
                property,
                Utc::now().into(),
            )?;
            let document = pdfmaker.generate(document)?;

            // Assign receipt external ID.
            receipt.external_id = Some(document.id);
            receipt.status = Some(document.status);

            // Create receipt.
            let receipt = match db.files().create(&receipt) {
                Ok(receipt) => receipt,
                Err(err) => return Err(err),
            };

            // Link receipt with rent.
            db.rents().update(&RentData {
                id: rent.id,
                receipt_id: Some(receipt.id),
                ..Default::default()
            })?;

            Ok(receipt)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(receipts)
}
