use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use crate::messenger::Messenger;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::LeaseId;
use trankeel_data::Tenant;
use trankeel_data::TenantId;

#[derive(Clone)]
pub struct LeaseAffected {
    pub lease_id: LeaseId,
    pub tenant_id: TenantId,
}

impl From<LeaseAffected> for Event {
    fn from(item: LeaseAffected) -> Self {
        Self::LeaseAffected(item)
    }
}

pub fn lease_affected(ctx: &Context, event: LeaseAffected) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let LeaseAffected {
        lease_id,
        tenant_id,
    } = event;

    let lease = db.leases().by_id(&lease_id)?;
    let tenant = db.tenants().by_id(&tenant_id)?;

    db.tenants().update(&Tenant {
        lease_id: Some(lease_id),
        ..tenant
    })?;

    let account = db.accounts().by_lease_id(&lease.id)?;
    let participant = db.persons().by_tenant_id(&tenant_id)?;
    let sender = db.persons().by_account_id_first(&account.id)?;
    let eventable = db.eventables().create(&Eventable::Lease(lease))?;

    messenger.message(
        db,
        EventType::LeaseCreated, // Use "LeaseCreated" as message event type.
        eventable.id(),
        account.id,
        sender.id,
        participant.id,
        None,
    )?;

    Ok(())
}
