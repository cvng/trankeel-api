use super::Account;
use super::File;
use super::FurnishedLeaseDetails;
use super::Property;
use super::Rent;
use super::Tenant;
use async_graphql::Context;
use async_graphql::Result;
use trankeel::AccountId;
use trankeel::Amount;
use trankeel::Client;
use trankeel::DateTime;
use trankeel::FileId;
use trankeel::FurnishedLeaseDuration;
use trankeel::LeaseId;
use trankeel::LeaseStatus;
use trankeel::LeaseType;
use trankeel::PropertyId;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Lease {
    pub id: LeaseId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub deposit_amount: Amount,
    pub effect_date: DateTime,
    pub signature_date: Option<DateTime>,
    pub rent_amount: Amount,
    pub rent_charges_amount: Option<Amount>,
    pub type_: LeaseType,
    pub lease_id: Option<FileId>,
    pub property_id: PropertyId,
    #[graphql(name = "data")]
    pub details: Option<FurnishedLeaseDetails>,
    pub expired_at: Option<DateTime>,
    pub renew_date: Option<DateTime>,
    pub duration: FurnishedLeaseDuration,
    //
    pub status: LeaseStatus,
    pub rent_full_amount: Amount,
}

#[async_graphql::ComplexObject]
impl Lease {
    async fn rents(&self, ctx: &Context<'_>) -> Result<Vec<Rent>> {
        Ok(ctx
            .data_unchecked::<Client>()
            .rents()
            .by_lease_id(&self.id)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn lease(&self) -> Option<File> {
        None
    }

    async fn tenants(&self, ctx: &Context<'_>) -> Result<Vec<Tenant>> {
        Ok(ctx
            .data_unchecked::<Client>()
            .tenants()
            .by_lease_id(&self.id)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn account(&self) -> Option<Account> {
        None
    }

    async fn property(&self, ctx: &Context<'_>) -> Result<Property> {
        Ok(ctx
            .data_unchecked::<Client>()
            .properties()
            .by_id(&self.property_id)?
            .into())
    }
}

impl From<trankeel::Lease> for Lease {
    fn from(item: trankeel::Lease) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            account_id: item.account_id,
            deposit_amount: item.deposit_amount,
            effect_date: item.effect_date,
            signature_date: item.signature_date,
            rent_amount: item.rent_amount,
            rent_charges_amount: item.rent_charges_amount,
            type_: item.type_,
            lease_id: item.lease_id,
            property_id: item.property_id,
            details: item.details.clone().map(Into::into),
            expired_at: item.expired_at,
            renew_date: item.renew_date,
            duration: item.duration,
            status: item.status(),
            rent_full_amount: item.rent_full_amount(),
        }
    }
}
