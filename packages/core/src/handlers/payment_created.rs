use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use crate::messenger::Messenger;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::Payment;

#[derive(Clone)]
pub struct PaymentCreated {
    pub payment: Payment,
}

impl From<PaymentCreated> for Event {
    fn from(item: PaymentCreated) -> Self {
        Event::PaymentCreated(item)
    }
}

pub fn payment_created(ctx: &Context, event: PaymentCreated) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let PaymentCreated { payment } = event;

    let account = db.accounts().by_payment_id(&payment.id)?;
    let participant = db.persons().by_payment_id(&payment.id)?;
    let eventable = db.eventables().create(&Eventable::Payment(payment))?;

    messenger.message(
        db,
        EventType::PaymentCreated,
        eventable.id(),
        account.id,
        participant.id,
        participant.id,
        None,
    )?;

    Ok(())
}
