use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use crate::mailer::Mailer;
use crate::templates::ReceiptCreatedMail;
use chrono::Utc;
use eyre::Error;
use trankeel_data::RentId;

#[derive(Clone)]
pub struct ReceiptSent {
    pub rent_id: RentId,
}

impl From<ReceiptSent> for Event {
    fn from(item: ReceiptSent) -> Self {
        Self::ReceiptSent(item)
    }
}

pub async fn receipt_sent(ctx: &Context, event: ReceiptSent) -> Result<()> {
    let db = ctx.db();
    let mailer = ctx.mailer();

    let ReceiptSent { rent_id } = event;

    let rent = db.rents().by_id(&rent_id)?;
    let lease = db.leases().by_id(&rent.lease_id)?;
    let tenants = db.tenants().by_lease_id(&lease.id)?;

    let receipt_id = if let Some(receipt_id) = rent.receipt_id {
        receipt_id
    } else if let Some(notice_id) = rent.notice_id {
        notice_id
    } else {
        return Err(Error::msg("receipt_or_notice_missing"));
    };

    let receipt = match db.files().by_id(&receipt_id) {
        Ok(receipt) => receipt,
        Err(err) => return Err(err),
    };

    mailer
        .batch(vec![ReceiptCreatedMail::try_new(
            &receipt,
            &rent,
            tenants,
            Utc::now().into(),
        )?])
        .await?;

    Ok(())
}
