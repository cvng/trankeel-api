use super::Account;
use super::Discussion;
use super::File;
use super::Lease;
use super::Payment;
use super::Property;
use super::Warrant;
use async_graphql::Context;
use async_graphql::Result;
use trankeel::AccountId;
use trankeel::Amount;
use trankeel::Client;
use trankeel::Date;
use trankeel::DateTime;
use trankeel::Email;
use trankeel::LeaseId;
use trankeel::Name;
use trankeel::PersonId;
use trankeel::PhoneNumber;
use trankeel::TenantId;
use trankeel::TenantStatus;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Tenant {
    pub id: TenantId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub person_id: PersonId,
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
    pub short_name: String,
    pub full_name: String,
    pub last_transaction: Option<Payment>,
    pub account: Option<Account>,
    pub property: Option<Property>,
    pub property_name: Option<String>,
    pub rent_payed_this_year: Option<String>,
    pub unpaid_rent_amount: Option<Amount>,
    pub files: Option<Vec<File>>,
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

    async fn discussion(&self, ctx: &Context<'_>) -> Result<Discussion> {
        Ok(ctx
            .data_unchecked::<Client>()
            .discussions()
            .by_initiator_id(&self.person_id)?
            .into())
    }

    async fn lease(&self, ctx: &Context<'_>) -> Result<Option<Lease>> {
        Ok(match self.lease_id {
            Some(lease_id) => Some(
                ctx.data_unchecked::<Client>()
                    .leases()
                    .by_id(&lease_id)?
                    .into(),
            ),
            None => None,
        })
    }
}

impl From<trankeel::Tenant> for Tenant {
    fn from(item: trankeel::Tenant) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            account_id: item.account_id,
            person_id: item.person_id,
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
            short_name: item.short_name(),
            full_name: item.full_name(),
            last_transaction: None,
            account: None,
            property: None,
            property_name: None,
            rent_payed_this_year: None,
            unpaid_rent_amount: None,
            files: None,
        }
    }
}
