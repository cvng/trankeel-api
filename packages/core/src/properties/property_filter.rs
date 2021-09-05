use crate::database::Db;
use crate::AuthId;
use eyre::Error;
use piteo_data::Property;
use piteo_data::PropertyId;

// # Queries

pub fn all_properties(
    db: impl Db,
    auth_id: AuthId,
    id: Option<PropertyId>,
) -> Result<Vec<Property>, Error> {
    db.properties().all(auth_id, id)
}
