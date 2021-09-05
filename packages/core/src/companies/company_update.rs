use crate::auth::AddressInput;

#[derive(async_graphql::InputObject)]
pub struct CompanyUpdateInput {
    address: Option<AddressInput>,
}
