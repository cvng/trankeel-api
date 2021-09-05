use crate::auth::AddressInput;
use async_graphql::InputObject;

#[derive(InputObject)]
pub struct CompanyUpdateInput {
    address: Option<AddressInput>,
}
