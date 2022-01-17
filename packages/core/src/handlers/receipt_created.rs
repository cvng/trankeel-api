use super::payment_created;
use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use crate::messenger::Messenger;
use crate::pdfmaker::Pdfmaker;
use crate::templates::ReceiptDocument;
use chrono::Utc;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_ops::event::PaymentCreated;
use trankeel_ops::event::ReceiptCreated;

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
    pdfmaker
        .generate(ReceiptDocument::try_new(
            receipt.clone(),
            rent.clone(),
            lender,
            tenants,
            property,
            Utc::now().into(),
        )?)
        .await?;

    Ok(())
}
