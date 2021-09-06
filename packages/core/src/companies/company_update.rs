use crate::auth::AddressInput;
use async_graphql::InputObject;

#[derive(InputObject)]
#[graphql(name = "CompanyUpdateInput")]
pub struct UpdateCompanyInput {
    address: Option<AddressInput>,
}
