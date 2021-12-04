use super::CreateProperty;
use super::CreatePropertyInput;
use crate::error::Result;
use trankeel_core::database::Db;
use trankeel_core::dispatcher;
use trankeel_core::dispatcher::Command;
use trankeel_core::dispatcher::Event;
use trankeel_core::handlers::PropertyCreated;
use trankeel_core::providers::Pg;
use trankeel_data::AuthId;
use trankeel_data::Property;

pub async fn create_property(
    db: &Pg,
    auth_id: &AuthId,
    input: CreatePropertyInput,
) -> Result<Property> {
    let account = db.accounts().by_auth_id(auth_id)?;

    let property = CreateProperty::new(&account)
        .run(input)
        .await
        .and_then(dispatcher::dispatch)?
        .iter()
        .find_map(|event| match event {
            Event::PropertyCreated(PropertyCreated { property }) => Some(property),
            _ => None,
        })
        .cloned()
        .unwrap();

    Ok(property)
}
