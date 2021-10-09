use super::Advertisement;
use super::Candidacy;
use super::Discussion;
use super::Event;
use super::File;
use super::Invoice;
use super::Lease;
use super::Lender;
use super::Payment;
use super::Person;
use super::Plan;
use super::Property;
use super::Rent;
use super::Summary;
use super::Tenant;
use crate::wip;
use async_graphql::Context;
use async_graphql::Result;
use piteo::AdvertisementId;
use piteo::AuthId;
use piteo::CandidacyId;
use piteo::Client;
use piteo::DateTime;
use piteo::DiscussionId;
use piteo::LeaseId;
use piteo::LenderId;
use piteo::PaymentId;
use piteo::PropertyId;
use piteo::TenantId;
use piteo::TenantStatus;

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn viewer(&self, ctx: &Context<'_>) -> Result<Person> {
        Ok(ctx
            .data_unchecked::<Client>()
            .persons()
            .by_auth_id(ctx.data::<AuthId>()?)?
            .into())
    }

    async fn advertisement(&self, ctx: &Context<'_>, id: AdvertisementId) -> Result<Advertisement> {
        Ok(ctx
            .data_unchecked::<Client>()
            .advertisements()
            .by_id(&id)?
            .into())
    }

    async fn candidacy(&self, ctx: &Context<'_>, id: CandidacyId) -> Result<Candidacy> {
        Ok(ctx
            .data_unchecked::<Client>()
            .candidacies()
            .by_id(&id)?
            .into())
    }

    async fn candidacies(
        &self,
        ctx: &Context<'_>,
        property_id: Option<PropertyId>,
    ) -> Result<Vec<Candidacy>> {
        if let Some(property_id) = property_id {
            Ok(ctx
                .data_unchecked::<Client>()
                .candidacies()
                .by_property_id(&property_id)?
                .into_iter()
                .map(Into::into)
                .collect())
        } else {
            Ok(ctx
                .data_unchecked::<Client>()
                .candidacies()
                .by_auth_id(ctx.data::<AuthId>()?)?
                .into_iter()
                .map(Into::into)
                .collect())
        }
    }

    async fn properties(
        &self,
        ctx: &Context<'_>,
        id: Option<PropertyId>,
        _query: Option<String>,
    ) -> Result<Vec<Property>> {
        if let Some(id) = id {
            Ok(vec![ctx
                .data_unchecked::<Client>()
                .properties()
                .by_id(&id)?
                .into()])
        } else {
            Ok(ctx
                .data_unchecked::<Client>()
                .properties()
                .by_auth_id(ctx.data::<AuthId>()?)?
                .into_iter()
                .map(Into::into)
                .collect())
        }
    }

    async fn tenants(
        &self,
        ctx: &Context<'_>,
        id: Option<TenantId>,
        _query: Option<String>,
        _status: Option<TenantStatus>,
    ) -> Result<Vec<Tenant>> {
        if let Some(id) = id {
            Ok(vec![ctx
                .data_unchecked::<Client>()
                .tenants()
                .by_id(&id)?
                .into()])
        } else {
            Ok(ctx
                .data_unchecked::<Client>()
                .tenants()
                .by_auth_id(ctx.data::<AuthId>()?)?
                .into_iter()
                .map(Into::into)
                .collect())
        }
    }

    async fn leases(
        &self,
        ctx: &Context<'_>,
        _id: Option<LeaseId>,
        _query: Option<String>,
    ) -> Result<Vec<Lease>> {
        Ok(ctx
            .data_unchecked::<Client>()
            .leases()
            .by_auth_id(ctx.data::<AuthId>()?)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn lenders(
        &self,
        ctx: &Context<'_>,
        id: Option<LenderId>,
        _query: Option<String>,
    ) -> Result<Vec<Lender>> {
        if let Some(id) = id {
            Ok(vec![ctx
                .data_unchecked::<Client>()
                .lenders()
                .by_id(&id)?
                .into()])
        } else {
            Ok(ctx
                .data_unchecked::<Client>()
                .lenders()
                .by_auth_id(ctx.data::<AuthId>()?)?
                .into_iter()
                .map(Into::into)
                .collect())
        }
    }

    async fn rents(
        &self,
        ctx: &Context<'_>,
        #[graphql(name = "since")] from: DateTime,
        #[graphql(name = "until")] to: DateTime,
    ) -> Result<Vec<Rent>> {
        Ok(ctx
            .data_unchecked::<Client>()
            .rents()
            .by_auth_id(ctx.data::<AuthId>()?)?
            .into_iter()
            .filter(|rent| rent.period_start.inner() >= from.inner())
            .filter(|rent| rent.period_start.inner() < to.inner())
            .map(Into::into)
            .collect())
    }

    async fn summary(
        &self,
        ctx: &Context<'_>,
        _since: DateTime,
        _until: DateTime,
    ) -> Result<Summary> {
        Ok(ctx
            .data_unchecked::<Client>()
            .reports()
            .by_auth_id(ctx.data::<AuthId>()?)?
            .into())
    }

    async fn events(&self, ctx: &Context<'_>) -> Result<Vec<Event>> {
        Ok(ctx
            .data_unchecked::<Client>()
            .events()
            .by_auth_id(ctx.data::<AuthId>()?)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn discussions(
        &self,
        ctx: &Context<'_>,
        id: Option<DiscussionId>,
    ) -> Result<Vec<Discussion>> {
        if let Some(id) = id {
            Ok(vec![ctx
                .data_unchecked::<Client>()
                .discussions()
                .by_id(&id)?
                .into()])
        } else {
            Ok(ctx
                .data_unchecked::<Client>()
                .discussions()
                .by_auth_id(ctx.data::<AuthId>()?)?
                .into_iter()
                .map(Into::into)
                .collect())
        }
    }

    async fn transactions(
        &self,
        _ctx: &Context<'_>,
        _id: Option<PaymentId>,
    ) -> Result<Vec<Payment>> {
        Err(wip())
    }

    async fn invoices(&self, _ctx: &Context<'_>) -> Result<Vec<Invoice>> {
        Err(wip())
    }

    async fn plans(&self, _ctx: &Context<'_>) -> Result<Vec<Plan>> {
        Err(wip())
    }

    async fn files(&self, _ctx: &Context<'_>) -> Result<Vec<File>> {
        Err(wip())
    }
}
