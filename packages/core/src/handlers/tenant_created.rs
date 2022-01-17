use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::TenantCreated;

pub fn tenant_created(ctx: &Context, event: TenantCreated) -> Result<()> {
    let db = ctx.db();

    let TenantCreated {
        tenant,
        identity,
        discussion,
        warrants,
    } = event;

    db.persons().create(&identity)?;
    db.tenants().create(&tenant)?;
    if let Some(discussion) = discussion {
        db.discussions().create(&discussion)?;
    }
    if let Some(warrants) = warrants {
        db.warrants().create_many(&warrants)?;
    }

    Ok(())
}
