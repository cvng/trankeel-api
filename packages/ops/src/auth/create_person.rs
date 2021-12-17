use super::AddressInput;
use crate::error::Result;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::Account;
use trankeel_data::Email;
use trankeel_data::Person;
use trankeel_data::PersonId;
use trankeel_data::PersonRole;
use trankeel_data::PhoneNumber;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreatePersonInput {
    pub email: Email,
    pub first_name: String,
    pub last_name: String,
    pub address: Option<AddressInput>,
    pub phone_number: Option<PhoneNumber>,
    pub role: PersonRole,
}

pub struct CreatePersonPayload {
    pub person: Person,
}

pub struct CreatePerson {
    account: Account,
}

impl CreatePerson {
    pub fn new(account: &Account) -> Self {
        Self {
            account: account.clone(),
        }
    }
}

impl Command for CreatePerson {
    type Input = CreatePersonInput;
    type Payload = CreatePersonPayload;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self { account } = self;

        let person = Person {
            id: PersonId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            account_id: account.id,
            auth_id: None,
            email: input.email,
            first_name: input.first_name,
            last_name: input.last_name,
            address: None,
            photo_url: None,
            role: input.role,
            phone_number: input.phone_number,
        };

        Ok(Self::Payload { person })
    }
}
