use crate::error::Result;
use crate::AuthId;
use trankeel_core::database::Db;
use trankeel_data::TenantId;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
pub struct DeleteTenantInput {
    pub id: TenantId,
}

// # Operation

pub fn delete_tenant(
    db: &impl Db,
    _auth_id: &AuthId,
    input: DeleteTenantInput,
) -> Result<TenantId> {
    input.validate()?;

    db.tenants().delete(input.id)?;

    Ok(input.id)
}
