use super::payment_created;
use super::PaymentCreated;
use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use crate::messenger::Messenger;
use crate::pdfmaker::Pdfmaker;
use crate::templates::ReceiptDocument;
use chrono::Utc;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::Payment;
use trankeel_data::Receipt;
use trankeel_data::Rent;

#[derive(Clone)]
pub struct ReceiptCreated {
    pub receipt: Receipt,
    pub rent: Rent,
    pub payment: Payment,
}

impl From<ReceiptCreated> for Event {
    fn from(item: ReceiptCreated) -> Self {
        Event::ReceiptCreated(item)
    }
}

pub fn receipt_created(ctx: &Context, event: ReceiptCreated) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let ReceiptCreated {
        receipt,
        rent,
        payment,
    } = event;

    db.files().create(&receipt)?;
    db.rents().update(&rent)?;

    payment_created(ctx, PaymentCreated { payment })?;

    let participant = db.persons().by_receipt_id(&receipt.id)?;

    messenger.message(
        EventType::ReceiptCreated,
        Eventable::File(receipt),
        participant.id,
        participant.id,
        None,
    )?;

    Ok(())
}

pub async fn receipt_created_async(ctx: &Context, event: ReceiptCreated) -> Result<()> {
    let db = ctx.db();
    let pdfmaker = ctx.pdfmaker();

    let ReceiptCreated { receipt, rent, .. } = event;

    let lease = db.leases().by_id(&rent.lease_id)?;
    let tenants = db.tenants().by_lease_id(&lease.id)?;
    let property = db.properties().by_id(&lease.property_id)?;
    let lender = db.lenders().by_id(&property.lender_id)?;

    // Try to generate receipt document (PDF).
    let document = pdfmaker
        .generate(ReceiptDocument::try_new(
            receipt.clone(),
            rent.clone(),
            lender,
            tenants,
            property,
            Utc::now().into(),
        )?)
        .await?;

    db.files().update(&Receipt {
        external_id: Some(document.id),
        ..receipt
    })?;

    Ok(())
}
