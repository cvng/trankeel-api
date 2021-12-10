use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use crate::messenger::Messenger;
use diesel::result::Error::NotFound;
use trankeel_data::Eventable;
use trankeel_data::Lease;
use trankeel_data::Rent;

#[derive(Clone)]
pub struct LeaseCreated {
    pub lease: Lease,
    pub rents: Vec<Rent>,
}

impl From<LeaseCreated> for Event {
    fn from(item: LeaseCreated) -> Self {
        Self::LeaseCreated(item)
    }
}

pub fn lease_created(ctx: &Context, event: LeaseCreated) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let LeaseCreated { lease, rents } = event.clone();

    db.leases().create(&lease)?;
    db.rents().create_many(&rents)?;

    let account = db.accounts().by_lease_id(&lease.id)?;
    let participant = db.persons().by_lease_id(&lease.id)?;
    let sender = db
        .persons()
        .by_account_id(&account.id)?
        .first()
        .cloned()
        .ok_or(NotFound)?;
    let eventable = db.eventables().create(&Eventable::Lease(lease))?;

    messenger.message(
        db,
        Event::LeaseCreated(event).into(),
        eventable.id(),
        account.id,
        sender.id,
        participant.id,
        None,
    )?;

    Ok(())
}
