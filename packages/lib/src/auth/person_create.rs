use super::AddressInput;
use async_graphql::InputObject;
use trankeel_data::Email;
use trankeel_data::Person;
use trankeel_data::PhoneNumber;

#[derive(InputObject)]
pub struct CreatePersonInput {
    pub email: Email,
    pub first_name: String,
    pub last_name: String,
    pub address: Option<AddressInput>,
    pub phone_number: Option<PhoneNumber>,
}

impl From<Person> for CreatePersonInput {
    fn from(item: Person) -> Self {
        Self {
            email: item.email,
            first_name: item.first_name,
            last_name: item.last_name,
            address: item.address.map(Into::into),
            phone_number: item.phone_number,
        }
    }
}
