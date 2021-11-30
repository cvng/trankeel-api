use crate::dispatcher::Event;
use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use crate::messenger;
use trankeel_data::Eventable;
use trankeel_data::Payment;

pub fn payment_created(ctx: &Context, event: &Event, payment: &Payment) -> Result<()> {
    let db = ctx.db();

    let account = db.accounts().by_payment_id(&payment.id)?;
    let participant = db.persons().by_payment_id(&payment.id)?;
    let eventable = db
        .eventables()
        .create(&Eventable::Payment(payment.clone()))?;

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
