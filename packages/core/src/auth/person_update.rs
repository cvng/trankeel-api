use super::AddressInput;
use async_graphql::InputObject;

#[derive(InputObject)]
pub struct UserUpdateInput {
    address: AddressInput,
    first_name: String,
    last_name: String,
}
