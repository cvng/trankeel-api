use crate::auth::AddressInput;
use async_graphql::InputObject;
use trankeel_data::Email;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateCompanyInput {
    pub address: AddressInput,
    pub email: Email,
    pub legal_entity: String,
}
