use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::TenantUpdated;

pub fn tenant_updated(ctx: &Context, event: TenantUpdated) -> Result<()> {
    let db = ctx.db();

    let TenantUpdated { tenant } = event;

    db.tenants().update(&tenant)?;

    Ok(())
}
