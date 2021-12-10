use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use trankeel_data::Discussion;
use trankeel_data::Person;
use trankeel_data::Tenant;
use trankeel_data::WarrantWithIdentity;

#[derive(Clone)]
pub struct TenantCreated {
    pub tenant: Tenant,
    pub identity: Person,
    pub discussion: Option<Discussion>,
    pub warrants: Option<Vec<WarrantWithIdentity>>,
}

impl From<TenantCreated> for Event {
    fn from(item: TenantCreated) -> Self {
        Self::TenantCreated(item)
    }
}

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
