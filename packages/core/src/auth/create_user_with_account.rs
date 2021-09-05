use crate::database::Db;
use crate::payment::PaymentProvider;
use eyre::Error;
use log::info;
use piteo_data::Account;
use piteo_data::AccountData;
use piteo_data::Address;
use piteo_data::AuthId;
use piteo_data::LenderData;
use piteo_data::Person;
use piteo_data::PersonData;
use validator::Validate;

// # Input

#[derive(Clone)]
pub struct AddressInput {
    pub city: String,
    pub country: Option<String>,
    pub line1: String,
    pub line2: Option<String>,
    pub postal_code: String,
}

#[derive(Clone, Validate)]
pub struct CreateUserWithAccountInput {
    pub address: Option<AddressInput>,
    pub auth_id: AuthId,
    #[validate(email)]
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub skip_create_customer: Option<bool>,
}

// # Operation

pub async fn create_user_with_account(
    db: impl Db,
    payment_provider: impl PaymentProvider,
    input: CreateUserWithAccountInput,
) -> Result<Person, Error> {
    input.validate()?;

    // Create user.
    let user = db.users().create(input.clone().into())?;

    // Create account.
    let account = db.accounts().create(AccountData {
        owner_id: user.id.to_string(),
    })?;

    // Update user account.
    let user = db.users().update(Person {
        account_id: Some(account.id),
        ..user
    })?;

    // Create lender.
    let _lender = db.lenders().create(LenderData {
        account_id: account.id,
        individual_id: Some(user.id),
        company_id: None,
    })?;

    if let Some(true) = input.skip_create_customer {
        return Ok(user);
    }

    // Create subscription.
    let subscription = payment_provider
        .create_subscription_with_customer(input.email)
        .await?;
    info!(
        "Created subscription {} for account {}",
        subscription.id, account.id
    );

    // Update the local customer data.
    db.accounts().update(Account {
        stripe_customer_id: Some(subscription.customer_id),
        stripe_subscription_id: Some(subscription.id),
        status: Some(subscription.status),
        trial_end: subscription.trial_end,
        ..account
    })?;

    Ok(user)
}

// # Utils

impl From<AddressInput> for Address {
    fn from(item: AddressInput) -> Self {
        Self {
            city: Some(item.city),
            country: item.country,
            line1: Some(item.line1),
            line2: item.line2,
            postal_code: Some(item.postal_code),
        }
    }
}

impl From<CreateUserWithAccountInput> for PersonData {
    fn from(item: CreateUserWithAccountInput) -> Self {
        Self {
            address: item.address.map(Into::into),
            auth_id: item.auth_id,
            email: item.email,
            first_name: item.first_name,
            last_name: item.last_name,
        }
    }
}
