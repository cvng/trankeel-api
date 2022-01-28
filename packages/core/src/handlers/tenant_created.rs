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

    if let Some(identity) = identity {
        db.persons().create(&identity).unwrap();
    }
    db.tenants().create(&tenant).unwrap();
    if let Some(discussion) = discussion {
        db.discussions().create(&discussion).unwrap();
    }
    if let Some(warrants) = warrants {
        db.warrants().create_many(&warrants).unwrap();
    }

    Ok(())
}
