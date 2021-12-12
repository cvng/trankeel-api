use crate::auth::AddressInput;
use async_graphql::InputObject;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct UpdatePersonInput {
    pub address: Option<AddressInput>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}
