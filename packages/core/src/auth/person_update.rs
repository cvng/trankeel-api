use super::AddressInput;

#[derive(async_graphql::InputObject)]
pub struct UserUpdateInput {
    address: AddressInput,
    first_name: String,
    last_name: String,
}
