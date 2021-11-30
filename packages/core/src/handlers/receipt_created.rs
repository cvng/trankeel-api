use crate::dispatcher::Event;
use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use crate::messenger;
use trankeel_data::Eventable;
use trankeel_data::Receipt;

pub fn receipt_created(ctx: &Context, event: &Event, receipt: &Receipt) -> Result<()> {
    let db = ctx.db();

    let account = db.accounts().by_receipt_id(&receipt.id)?;
    let participant = db.persons().by_receipt_id(&receipt.id)?;
    let eventable = db.eventables().create(&Eventable::File(receipt.clone()))?;

    messenger::message(
        db,
        event.clone().into(),
        eventable.id(),
        account.id,
        participant.id,
        participant.id,
        None,
    )?;

    Ok(())
}
