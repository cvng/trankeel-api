use crate::error::Result;
use crate::AuthId;
use trankeel_core::database::Db;
use trankeel_data::LeaseId;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
pub struct DeleteLeaseInput {
    pub id: LeaseId,
}

// # Operation

pub fn delete_lease(db: &impl Db, _auth_id: &AuthId, input: DeleteLeaseInput) -> Result<LeaseId> {
    input.validate()?;

    db.leases().delete(input.id)?;

    Ok(input.id)
}
