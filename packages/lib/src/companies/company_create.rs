use crate::auth::AddressInput;
use async_graphql::InputObject;
use piteo_data::Email;

#[derive(InputObject)]
pub struct CompanyInput {
    address: AddressInput,
    email: Email,
    legal_entity: String,
}
