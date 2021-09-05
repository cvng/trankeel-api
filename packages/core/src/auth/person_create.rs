use super::AddressInput;
use piteo_data::Email;

#[derive(async_graphql::InputObject)]
pub struct UserInput {
    address: AddressInput,
    email: Email,
    first_name: String,
    last_name: String,
}
