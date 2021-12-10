use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use trankeel_data::Tenant;

#[derive(Clone)]
pub struct TenantUpdated {
    pub tenant: Tenant,
}

impl From<TenantUpdated> for Event {
    fn from(item: TenantUpdated) -> Self {
        Self::TenantUpdated(item)
    }
}

pub fn tenant_updated(ctx: &Context, event: TenantUpdated) -> Result<()> {
    let db = ctx.db();

    let TenantUpdated { tenant } = event;

    db.tenants().update(&tenant)?;

    Ok(())
}
