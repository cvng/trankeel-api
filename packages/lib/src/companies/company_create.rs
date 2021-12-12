use crate::auth::AddressInput;
use async_graphql::InputObject;
use trankeel_data::Email;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateCompanyInput {
    address: AddressInput,
    email: Email,
    legal_entity: String,
}
