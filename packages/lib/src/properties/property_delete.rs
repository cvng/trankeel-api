use crate::error::Result;
use crate::AuthId;
use trankeel_core::database::Db;
use trankeel_data::PropertyId;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
pub struct DeletePropertyInput {
    pub id: PropertyId,
}

// # Operation

pub fn delete_property(
    db: &impl Db,
    _auth_id: &AuthId,
    input: DeletePropertyInput,
) -> Result<PropertyId> {
    input.validate()?;

    db.properties().delete(input.id)?;

    Ok(input.id)
}
