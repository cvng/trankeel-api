use super::AddressInput;
use async_graphql::InputObject;
use piteo_data::Email;
use piteo_data::PhoneNumber;

#[derive(InputObject)]
pub struct CreatePersonInput {
    pub email: Email,
    pub first_name: String,
    pub last_name: String,
    pub address: AddressInput,
    pub phone_number: Option<PhoneNumber>,
}
