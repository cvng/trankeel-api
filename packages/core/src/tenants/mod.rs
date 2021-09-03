pub mod ops;

use crate::database::Db;
use crate::AuthId;
use eyre::Error;
use piteo_data::Tenant;
use piteo_data::TenantId;

// # Queries

pub fn all_tenants<'a>(
    db: impl Db<'a>,
    auth_id: AuthId,
    id: Option<TenantId>,
) -> Result<Vec<Tenant>, Error> {
    db.tenants().all(auth_id, id)
}
