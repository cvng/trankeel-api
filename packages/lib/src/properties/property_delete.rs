use crate::AuthId;
use piteo_core::database::Db;
use piteo_core::error::Error;
use piteo_data::PropertyId;
use validator::Validate;

// # Input

#[derive(Validate)]
pub struct DeletePropertyInput {
    pub id: PropertyId,
}

// # Operation

pub fn delete_property(
    db: &impl Db,
    _auth_id: &AuthId,
    input: DeletePropertyInput,
) -> Result<PropertyId, Error> {
    input.validate()?;

    db.properties().delete(input.id)?;

    Ok(input.id)
}