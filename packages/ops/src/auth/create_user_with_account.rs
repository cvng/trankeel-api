use crate::error::Result;
use async_graphql::InputObject;
use crate::Command;
use trankeel_data::Account;
use trankeel_data::AccountId;
use trankeel_data::AccountStatus;
use trankeel_data::Address;
use trankeel_data::AuthId;
use trankeel_data::Lender;
use trankeel_data::LenderId;
use trankeel_data::Person;
use trankeel_data::PersonId;
use trankeel_data::PersonRole;
use validator::Validate;

#[derive(Clone, InputObject)]
pub struct AddressInput {
    pub city: String,
    pub country: Option<String>,
    pub line1: String,
    pub line2: Option<String>,
    pub postal_code: String,
}

#[derive(InputObject, Validate)]
pub struct CreateUserWithAccountInput {
    pub address: Option<AddressInput>,
    pub auth_id: AuthId,
    #[validate(email)]
    pub email: String, // Email,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: Option<String>,
    pub skip_create_customer: Option<bool>,
}

pub struct CreateUserWithAccountPayload {
    pub user: Person,
    pub lender: Lender,
    pub account: Account,
}

pub struct CreateUserWithAccount;

impl CreateUserWithAccount {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self
    }
}

impl Command for CreateUserWithAccount {
    type Input = CreateUserWithAccountInput;
    type Payload = CreateUserWithAccountPayload;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        // Create account.
        let account = Account {
            id: AccountId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            plan_id: None,
            status: AccountStatus::default(),
            stripe_customer_id: None,
            stripe_subscription_id: None,
            trial_end: None,
        };

        // Create user.
        let user = Person {
            id: PersonId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            account_id: account.id,
            auth_id: Some(input.auth_id),
            email: input.email.clone().into(),
            first_name: input.first_name,
            last_name: input.last_name,
            address: input.address.map(Into::into),
            photo_url: None,
            role: PersonRole::User,
            phone_number: input.phone_number.map(Into::into),
        };

        // Create lender.
        let lender = Lender {
            id: LenderId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            account_id: account.id,
            individual_id: Some(user.id),
            company_id: None,
        };

        Ok(Self::Payload {
            user,
            lender,
            account,
        })
    }
}

impl From<AddressInput> for Address {
    fn from(item: AddressInput) -> Self {
        Self {
            city: item.city,
            country: item.country,
            line1: item.line1,
            line2: item.line2,
            postal_code: item.postal_code,
        }
    }
}

impl From<Address> for AddressInput {
    fn from(item: Address) -> Self {
        Self {
            city: item.city,
            country: item.country,
            line1: item.line1,
            line2: item.line2,
            postal_code: item.postal_code,
        }
    }
}
