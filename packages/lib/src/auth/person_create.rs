use super::AddressInput;
use async_graphql::InputObject;
use trankeel_data::Email;
use trankeel_data::PhoneNumber;

#[derive(InputObject)]
pub struct CreatePersonInput {
    pub email: Email,
    pub first_name: String,
    pub last_name: String,
    pub address: Option<AddressInput>,
    pub phone_number: Option<PhoneNumber>,
}
