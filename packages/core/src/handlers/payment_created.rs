use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use crate::messenger::Messenger;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_ops::event::PaymentCreated;

pub fn payment_created(ctx: &Context, event: PaymentCreated) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let PaymentCreated { payment } = event;

    db.payments().create(&payment)?;

    let participant = db.persons().by_payment_id(&payment.id)?;

    messenger.message(
        EventType::PaymentCreated,
        Eventable::Payment(payment),
        participant.id,
        participant.id,
        None,
    )?;

    Ok(())
}
