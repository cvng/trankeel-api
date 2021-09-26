use crate::AuthId;
use piteo_core::database::Db;
use piteo_core::error::Error;
use piteo_data::TenantId;
use validator::Validate;

// # Input

#[derive(Validate)]
pub struct DeleteTenantInput {
    pub id: TenantId,
}

// # Operation

pub fn delete_tenant(
    db: &impl Db,
    _auth_id: &AuthId,
    input: DeleteTenantInput,
) -> Result<TenantId, Error> {
    input.validate()?;

    db.tenants().delete(input.id)?;

    Ok(input.id)
}