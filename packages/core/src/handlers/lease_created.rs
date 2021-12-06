use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use crate::messenger::Messenger;
use diesel::result::Error::NotFound;
use trankeel_data::Eventable;
use trankeel_data::Lease;

pub fn lease_created(ctx: &Context, event: &Event, lease: &Lease) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let account = db.accounts().by_lease_id(&lease.id)?;
    let participant = db.persons().by_lease_id(&lease.id)?;
    let sender = db
        .persons()
        .by_account_id(&account.id)?
        .first()
        .cloned()
        .ok_or(NotFound)?;
    let eventable = db.eventables().create(&Eventable::Lease(lease.clone()))?;

    messenger.message(
        db,
        event.clone().into(),
        eventable.id(),
        account.id,
        sender.id,
        participant.id,
        None,
    )?;

    Ok(())
}
