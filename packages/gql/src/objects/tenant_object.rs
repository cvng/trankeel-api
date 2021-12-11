use super::Discussion;
use super::Lease;
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
    pub lease_id: Option<LeaseId>,
    pub is_student: Option<bool>,
    //
    pub display_name: String,
    pub short_name: String,
    pub full_name: String,
    pub status: TenantStatus,
    pub balance: Option<Amount>,
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
            lease_id: item.lease_id,
            is_student: item.is_student,
            display_name: item.display_name(),
            short_name: item.short_name(),
            full_name: item.full_name(),
            status: Default::default(),
            balance: Default::default(),
        }
    }
}

impl From<trankeel::TenantWithBalance> for Tenant {
    fn from(item: trankeel::TenantWithBalance) -> Self {
        Self {
            id: item.0.id,
            created_at: item.0.created_at,
            updated_at: item.0.updated_at,
            account_id: item.0.account_id,
            person_id: item.0.person_id,
            birthdate: item.0.birthdate,
            birthplace: item.0.birthplace.clone(),
            email: item.0.email.clone(),
            first_name: item.0.first_name.clone(),
            last_name: item.0.last_name.clone(),
            note: item.0.note.clone(),
            phone_number: item.0.phone_number.clone(),
            lease_id: item.0.lease_id,
            is_student: item.0.is_student,
            display_name: item.0.display_name(),
            short_name: item.0.short_name(),
            full_name: item.0.full_name(),
            status: item.0.status(item.1.clone()),
            balance: Some(item.1.balance),
        }
    }
}
