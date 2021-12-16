use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use crate::messenger::Messenger;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::Receipt;

#[derive(Clone)]
pub struct ReceiptCreated {
    pub receipt: Receipt,
}

impl From<ReceiptCreated> for Event {
    fn from(item: ReceiptCreated) -> Self {
        Event::ReceiptCreated(item)
    }
}

pub fn receipt_created(ctx: &Context, event: ReceiptCreated) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let ReceiptCreated { receipt } = event;

    let account = db.accounts().by_receipt_id(&receipt.id)?;
    let participant = db.persons().by_receipt_id(&receipt.id)?;
    let eventable = db.eventables().create(&Eventable::File(receipt))?;

    messenger.message(
        db,
        EventType::ReceiptCreated,
        eventable.id(),
        account.id,
        participant.id,
        participant.id,
        None,
    )?;

    Ok(())
}
