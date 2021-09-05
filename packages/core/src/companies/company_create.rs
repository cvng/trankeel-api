use crate::auth::AddressInput;
use piteo_data::Email;

#[derive(async_graphql::InputObject)]
pub struct CompanyInput {
    address: AddressInput,
    email: Email,
    legal_entity: String,
}
