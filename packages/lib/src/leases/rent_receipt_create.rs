use crate::error::Result;
use async_graphql::InputObject;
use chrono::Utc;
use trankeel_core::database::Db;
use trankeel_core::dispatcher::dispatch;
use trankeel_core::dispatcher::Event;
use trankeel_core::handlers::ReceiptSent;
use trankeel_core::pdfmaker::Pdfmaker;
use trankeel_core::templates::ReceiptDocument;
use trankeel_data::receipt_filename;
use trankeel_data::AuthId;
use trankeel_data::DateTime;
use trankeel_data::FileType;
use trankeel_data::Payment;
use trankeel_data::PaymentId;
use trankeel_data::Receipt;
use trankeel_data::ReceiptId;
use trankeel_data::Rent;
use trankeel_data::RentId;
use trankeel_data::RentStatus;
use trankeel_data::TransactionType;
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
    _auth_id: &AuthId,
    pdfmaker: &impl Pdfmaker,
    input: CreateReceiptsInput,
) -> Result<Vec<Receipt>> {
    input.validate()?;

    let rents = setlle_rents(db, input.rent_ids)?;

    let receipts = generate_receipts(db, pdfmaker, rents).await?;

    Ok(receipts)
}

pub fn send_receipts(input: SendReceiptsInput) -> Result<Vec<Event>> {
    input.validate()?;

    let SendReceiptsInput { rent_ids } = input;

    Ok(rent_ids
        .into_iter()
        .map(|rent_id| ReceiptSent { rent_id }.into())
        .collect())
}

// # Utils

fn setlle_rents(db: &impl Db, rent_ids: Vec<RentId>) -> Result<Vec<Rent>> {
    let mut rents = vec![];

    for rent_id in rent_ids {
        let rent = db.rents().by_id(&rent_id)?;

        let rent = db.rents().update(&Rent {
            id: rent_id,
            status: RentStatus::Paid,
            ..rent
        })?;

        let payment = db.payments().create(&Payment {
            id: PaymentId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            rent_id,
            amount: rent.full_amount,
            date: Utc::now().into(),
            type_: TransactionType::Rent,
            label: None,
        })?;

        rents.push(rent);

        dispatch(vec![Event::PaymentCreated(payment)])?;
    }

    Ok(rents)
}

async fn generate_receipts(
    db: &impl Db,
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
        let receipt_id = ReceiptId::new();
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
        let receipt = match db.files().create(&receipt) {
            Ok(receipt) => receipt,
            Err(err) => return Err(err),
        };

        // Link receipt with rent.
        db.rents().update(&Rent {
            id: rent.id,
            receipt_id: Some(receipt.id),
            ..rent
        })?;

        receipts.push(receipt.clone());

        dispatch(vec![Event::ReceiptCreated(receipt)])?;
    }

    Ok(receipts)
}
