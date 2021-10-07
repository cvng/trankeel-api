use super::Account;
use super::File;
use super::Lease;
use super::Payment;
use super::Property;
use super::Warrant;
use async_graphql::Context;
use async_graphql::Result;
use piteo::AccountId;
use piteo::Amount;
use piteo::Client;
use piteo::Date;
use piteo::DateTime;
use piteo::Email;
use piteo::LeaseId;
use piteo::Name;
use piteo::PersonId;
use piteo::PhoneNumber;
use piteo::TenantId;
use piteo::TenantStatus;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Tenant {
    pub id: TenantId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub person_id: PersonId,
    pub apl: Option<bool>,
    pub birthdate: Option<Date>,
    pub birthplace: Option<String>,
    pub email: Email,
    pub first_name: String,
    pub last_name: String,
    pub note: Option<String>,
    pub phone_number: Option<PhoneNumber>,
    pub status: TenantStatus,
    pub lease_id: Option<LeaseId>,
    pub is_student: Option<bool>,
    //
    pub display_name: String,
    pub account: Option<Account>,
    pub property: Option<Property>,
    pub full_name: Option<String>,
    pub short_name: Option<String>,
    pub last_transaction: Option<Payment>,
    pub property_name: Option<String>,
    pub rent_payed_this_year: Option<String>,
    pub unpaid_rent_amount: Option<Amount>,
    pub files: Option<Vec<File>>,
    pub lease: Option<Lease>,
}

#[async_graphql::ComplexObject]
impl Tenant {
    async fn warrants(&self, ctx: &Context<'_>) -> Result<Vec<Warrant>> {
        Ok(ctx
            .data_unchecked::<Client>()
            .warrants()
            .by_tenant_id(&self.id)?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}

impl From<piteo::Tenant> for Tenant {
    fn from(item: piteo::Tenant) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            account_id: item.account_id,
            person_id: item.person_id,
            apl: item.apl,
            birthdate: item.birthdate,
            birthplace: item.birthplace.clone(),
            email: item.email.clone(),
            first_name: item.first_name.clone(),
            last_name: item.last_name.clone(),
            note: item.note.clone(),
            phone_number: item.phone_number.clone(),
            status: item.status,
            lease_id: item.lease_id,
            is_student: item.is_student,
            display_name: item.display_name(),
            account: None,
            property: None,
            full_name: None,
            short_name: None,
            last_transaction: None,
            property_name: None,
            rent_payed_this_year: None,
            unpaid_rent_amount: None,
            files: None,
            lease: None,
        }
    }
}
