use crate::error::Result;
use crate::event::AccountCreated;
use crate::event::Event;
use crate::event::LenderCreated;
use crate::event::PersonCreated;
use crate::event::SubscriptionRequested;
use crate::Command;
use async_graphql::InputObject;
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

pub struct CreateUserWithAccount {
    user_id: PersonId,
    lender_id: LenderId,
    account_id: AccountId,
}

impl CreateUserWithAccount {
    pub fn new(user_id: PersonId, lender_id: LenderId, account_id: AccountId) -> Self {
        Self {
            user_id,
            lender_id,
            account_id,
        }
    }
}

impl Command for CreateUserWithAccount {
    type Input = CreateUserWithAccountInput;
    type Payload = Vec<Event>;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self {
            user_id,
            lender_id,
            account_id,
        } = self;

        // Create account.
        let account = Account {
            id: account_id,
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
            id: user_id,
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
            id: lender_id,
            created_at: Default::default(),
            updated_at: Default::default(),
            account_id: account.id,
            individual_id: Some(user.id),
            company_id: None,
        };

        let mut events = vec![
            AccountCreated {
                account: account.clone(),
            }
            .into(),
            PersonCreated {
                person: user.clone(),
            }
            .into(),
            LenderCreated { lender }.into(),
        ];

        if !matches!(input.skip_create_customer, Some(true)) {
            events.push(
                SubscriptionRequested {
                    account_id: account.id,
                    email: user.email, // TODO: use account email for billing
                }
                .into(),
            )
        }

        Ok(events)
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
