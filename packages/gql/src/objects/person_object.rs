use super::Account;
use super::Address;
use async_graphql::Context;
use async_graphql::Result;
use trankeel::AccountId;
use trankeel::AuthId;
use trankeel::Client;
use trankeel::DateTime;
use trankeel::Email;
use trankeel::Name;
use trankeel::PersonId;
use trankeel::PersonRole;
use trankeel::PhoneNumber;
use trankeel::Url;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Person {
    pub id: PersonId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub auth_id: Option<AuthId>,
    pub email: Email,
    pub first_name: String,
    pub last_name: String,
    pub address: Option<Address>,
    pub photo_url: Option<Url>,
    pub role: PersonRole,
    pub phone_number: Option<PhoneNumber>,
    //
    pub display_name: String,
}

#[async_graphql::ComplexObject]
impl Person {
    async fn account(&self, ctx: &Context<'_>) -> Result<Account> {
        Ok(ctx
            .data_unchecked::<Client>()
            .accounts()
            .by_id(&self.account_id)?
            .into())
    }

    async fn accounts(&self) -> Option<Vec<Account>> {
        None
    }
}

impl From<trankeel::Person> for Person {
    fn from(item: trankeel::Person) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            account_id: item.account_id,
            auth_id: item.auth_id.clone(),
            email: item.email.clone(),
            first_name: item.first_name.clone(),
            last_name: item.last_name.clone(),
            address: item.address.clone().map(Into::into),
            photo_url: item.photo_url.clone(),
            role: item.role,
            phone_number: item.phone_number.clone(),
            display_name: item.display_name(),
        }
    }
}
