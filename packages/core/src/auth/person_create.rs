use super::AddressInput;
use async_graphql::InputObject;
use piteo_data::Email;

#[derive(InputObject)]
pub struct UserInput {
    address: AddressInput,
    email: Email,
    first_name: String,
    last_name: String,
}
