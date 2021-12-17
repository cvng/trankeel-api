use crate::auth::AddressInput;
use async_graphql::InputObject;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct UpdateCompanyInput {
    pub address: Option<AddressInput>,
}
