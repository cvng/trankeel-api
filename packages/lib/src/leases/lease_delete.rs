use crate::error::Result;
use crate::AuthId;
use piteo_core::database::Db;
use piteo_data::LeaseId;
use validator::Validate;

// # Input

#[derive(Validate)]
pub struct DeleteLeaseInput {
    pub id: LeaseId,
}

// # Operation

pub fn delete_lease(db: &impl Db, _auth_id: &AuthId, input: DeleteLeaseInput) -> Result<LeaseId> {
    input.validate()?;

    db.leases().delete(input.id)?;

    Ok(input.id)
}
