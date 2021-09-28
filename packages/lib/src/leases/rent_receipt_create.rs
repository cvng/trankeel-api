use crate::documents::ReceiptDocument;
use crate::error::Result;
use crate::messages::ReceiptMail;
use async_graphql::InputObject;
use chrono::Utc;
use piteo_core::activity::trace;
use piteo_core::database::Db;
use piteo_core::error::Error;
use piteo_core::mailer::Mailer;
use piteo_core::pdfmaker::Pdfmaker;
use piteo_data::receipt_filename;
use piteo_data::AuthId;
use piteo_data::DateTime;
use piteo_data::EventType;
use piteo_data::FileType;
use piteo_data::Payment;
use piteo_data::PaymentId;
use piteo_data::Receipt;
use piteo_data::ReceiptId;
use piteo_data::Rent;
use piteo_data::RentData;
use piteo_data::RentId;
use piteo_data::RentStatus;
use piteo_data::TransactionType;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
#[graphql(name = "RentReceiptInput")]
pub struct CreateReceiptsInput {
    rent_ids: Vec<RentId>,
    date: Option<DateTime>,
    send_mail: Option<bool>,
}

#[derive(InputObject, Validate)]
pub struct SendReceiptsInput {
    pub rent_ids: Vec<RentId>,
}

// # Operation

pub async fn create_receipts(
    db: &impl Db,
    auth_id: &AuthId,
    pdfmaker: &impl Pdfmaker,
    input: CreateReceiptsInput,
) -> Result<Vec<Receipt>> {
    input.validate()?;

    let rents = setlle_rents(db, auth_id, input.rent_ids)?;

    let receipts = generate_receipts(db, auth_id, pdfmaker, rents).await?;

    Ok(receipts)
}

pub async fn send_receipts(
    db: &impl Db,
    auth_id: &AuthId,
    mailer: &impl Mailer,
    input: SendReceiptsInput,
) -> Result<Vec<Receipt>> {
    input.validate()?;

    let mut receipts = vec![];

    for rent_id in input.rent_ids {
        let rent = db.rents().by_id(&rent_id)?;
        let lease = db.leases().by_id(&rent.lease_id)?;
        let tenants = db.tenants().by_lease_id(&lease.id)?;

        let receipt_id = if let Some(receipt_id) = rent.receipt_id {
            receipt_id
        } else if let Some(notice_id) = rent.notice_id {
            notice_id
        } else {
            return Err(Error::msg("not found"));
        };
        let receipt = match db.files().by_id(&receipt_id) {
            Ok(receipt) => receipt,
            Err(err) => return Err(err),
        };
        receipts.push(receipt.clone());

        let mail = ReceiptMail::try_new(&receipt, &rent, tenants, Utc::now().into())?;
        mailer.batch(vec![mail]).await?;

        match receipt.type_ {
            FileType::RentReceipt => trace(db, auth_id, EventType::RentReceiptSent, rent.id).ok(),
            _ => trace(db, auth_id, EventType::PaymentNoticeSent, rent.id).ok(),
        };
    }

    Ok(receipts)
}

// # Utils

fn setlle_rents(db: &impl Db, auth_id: &AuthId, rent_ids: Vec<RentId>) -> Result<Vec<Rent>> {
    let mut rents = vec![];

    for rent_id in rent_ids {
        let rent = db.rents().update(RentData {
            id: rent_id,
            status: Some(RentStatus::Paid),
            ..Default::default()
        })?;

        let payment = db.payments().create(Payment {
            id: PaymentId::new_v4(),
            created_at: Default::default(),
            updated_at: Default::default(),
            rent_id,
            amount: rent.full_amount,
            date: Utc::now().into(),
            type_: TransactionType::Rent,
            label: None,
        })?;

        rents.push(rent);

        trace(db, auth_id, EventType::PaymentCreated, payment.id).ok();
    }

    Ok(rents)
}

async fn generate_receipts(
    db: &impl Db,
    auth_id: &AuthId,
    pdfmaker: &impl Pdfmaker,
    rents: Vec<Rent>,
) -> Result<Vec<Receipt>> {
    let mut receipts = vec![];

    for rent in rents {
        // Try to fetch associated entities.
        let lease = db.leases().by_id(&rent.lease_id)?;
        let tenants = db.tenants().by_lease_id(&lease.id)?;
        let property = db.properties().by_id(&lease.property_id)?;
        let lender = db.lenders().by_id(&property.lender_id)?;

        // Init new receipt.
        let receipt_id = ReceiptId::new_v4();
        let mut receipt = Receipt {
            id: receipt_id,
            type_: FileType::RentReceipt,
            filename: Some(receipt_filename(&receipt_id, &rent)),
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
        let document = pdfmaker.generate(document).await?;

        // Assign receipt external ID.
        receipt.external_id = Some(document.id);
        receipt.status = Some(document.status);

        // Create receipt.
        let receipt = match db.files().create(receipt) {
            Ok(receipt) => receipt,
            Err(err) => return Err(err),
        };

        // Link receipt with rent.
        db.rents().update(RentData {
            id: rent.id,
            receipt_id: Some(receipt.id),
            ..Default::default()
        })?;

        receipts.push(receipt);

        trace(db, auth_id, EventType::RentReceiptCreated, rent.id).ok();
    }

    Ok(receipts)
}
