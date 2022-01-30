use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::TenantDeleted;

pub fn tenant_deleted(ctx: &Context, event: TenantDeleted) -> Result<()> {
    let db = ctx.db();

    let TenantDeleted { tenant_id } = event;

    db.tenants().delete(&tenant_id)?;

    Ok(())
}
