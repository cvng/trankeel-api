use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use crate::messenger::Messenger;
use trankeel_data::Eventable;
use trankeel_ops::event::Event;
use trankeel_ops::event::PaymentCreated;

pub fn payment_created(ctx: &Context, event: PaymentCreated) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let PaymentCreated { payment } = event.clone();

    db.payments().create(&payment)?;

    let participant = db.persons().by_payment_id(&payment.id)?;

    messenger.message(
        participant.id,
        participant.id,
        None,
        Some(Event::from(event).event_type()),
        Some(Eventable::Payment(payment)),
    )?;

    Ok(())
}
