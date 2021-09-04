use crate::database::Db;
use eyre::Error;
use piteo_data::AccountData;
use piteo_data::Address;
use piteo_data::AuthId;
use piteo_data::LenderData;
use piteo_data::Person;
use piteo_data::PersonData;
use validator::Validate;

#[derive(Clone)]
pub struct AddressInput {
    pub city: String,
    pub country: Option<String>,
    pub line1: String,
    pub line2: Option<String>,
    pub postal_code: String,
}

#[derive(Clone, Validate)]
pub struct UserWithAccountInput {
    pub address: Option<AddressInput>,
    pub auth_id: AuthId,
    #[validate(email)]
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub skip_create_customer: Option<bool>,
}

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

impl From<UserWithAccountInput> for PersonData {
    fn from(item: UserWithAccountInput) -> Self {
        Self {
            address: item.address.map(Into::into),
            auth_id: item.auth_id,
            email: item.email,
            first_name: item.first_name,
            last_name: item.last_name,
        }
    }
}

pub fn create_user_with_account<'a>(
    db: impl Db<'a>,
    input: UserWithAccountInput,
) -> Result<Person, Error> {
    input.validate()?;

    // Create user.
    let mut user = db.users().create(input.clone().into())?;

    // Create account.
    let account = db.accounts().create(AccountData {
        owner_id: user.id.to_string(),
    })?;

    // Update user account.
    user.account_id = Some(account.id);
    let user = db.users().update(user)?;

    // Create lender.
    let _lender = db.lenders().create(LenderData {
        account_id: account.id,
        individual_id: Some(user.id),
        company_id: None,
    })?;

    if let Some(true) = input.skip_create_customer {
        // TODO: Create stripe user. https://crates.io/crates/stripe-rust
    }

    Ok(user)
}
