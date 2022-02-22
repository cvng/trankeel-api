use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use crate::messenger::Messenger;
use trankeel_data::Eventable;
use trankeel_ops::event::Event;
use trankeel_ops::event::LeaseAffected;

pub fn lease_affected(ctx: &Context, event: LeaseAffected) -> Result<()> {
    let db = ctx.db();

    let LeaseAffected {
        lease_id,
        tenant_id,
    } = event;

    let mut tenant = db.tenants().by_id(&tenant_id)?;

    // Update tenant status.
    tenant.lease_id = Some(lease_id);

    db.tenants().update(&tenant)?;

    Ok(())
}

pub async fn lease_affected_async(ctx: &Context, event: LeaseAffected) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let LeaseAffected {
        lease_id,
        tenant_id,
    } = event;

    let lease = db.leases().by_id(&lease_id)?;
    let account = db.accounts().by_lease_id(&lease_id)?;
    let participant = db.persons().by_tenant_id(&tenant_id)?;
    let sender = db.persons().by_account_id_first(&account.id)?;

    messenger.message(
        sender.id,
        participant.id,
        None,
        Some(Event::from(event).event_type()),
        Some(Eventable::Lease(lease)),
    )?;

    Ok(())
}
