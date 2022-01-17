use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use crate::messenger::Messenger;
use diesel::result::Error::NotFound;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_ops::event::LeaseAffected;

pub fn lease_affected(ctx: &Context, event: LeaseAffected) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let LeaseAffected { tenant } = event;

    let lease = db.leases().by_id(&tenant.lease_id.ok_or(NotFound)?)?;

    db.tenants().update(&tenant)?;

    let account = db.accounts().by_lease_id(&lease.id)?;
    let participant = db.persons().by_tenant_id(&tenant.id)?;
    let sender = db.persons().by_account_id_first(&account.id)?;

    messenger.message(
        EventType::LeaseCreated, // Use "LeaseCreated" as message event type.
        Eventable::Lease(lease),
        sender.id,
        participant.id,
        None,
    )?;

    Ok(())
}
