use crate::objects::Advertisement;
use crate::objects::Candidacy;
use crate::objects::Event;
use crate::objects::File;
use crate::objects::Invoice;
use crate::objects::Lease;
use crate::objects::Lender;
use crate::objects::Payment;
use crate::objects::Person;
use crate::objects::Plan;
use crate::objects::Property;
use crate::objects::Rent;
use crate::objects::Summary;
use crate::objects::Tenant;
use async_graphql::Context;
use async_graphql::Result;
use async_graphql::ID;
use piteo::db;
use piteo::AdvertisementId;
use piteo::AuthId;
use piteo::DateTime;
use piteo::Db;
use piteo::LenderId;
use piteo::PropertyId;
use piteo::TenantId;
use piteo::TenantStatus;

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn viewer(&self, ctx: &Context<'_>) -> Result<Person> {
        Ok(db(&ctx.into())
            .persons()
            .by_auth_id(ctx.data::<AuthId>()?)
            .map(Person::from)?)
    }

    async fn advertisement(&self, ctx: &Context<'_>, id: ID) -> Result<Advertisement> {
        Ok(db(&ctx.into())
            .advertisements()
            .by_id(&id.parse::<AdvertisementId>()?)?
            .into())
    }

    async fn candidacies(
        &self,
        ctx: &Context<'_>,
        property_id: Option<ID>,
    ) -> Result<Vec<Candidacy>> {
        if let Some(property_id) = property_id {
            Ok(db(&ctx.into())
                .candidacies()
                .by_property_id(&property_id.parse::<PropertyId>()?)
                .and_then(map_res)?)
        } else {
            Ok(db(&ctx.into())
                .candidacies()
                .by_auth_id(ctx.data::<AuthId>()?)
                .and_then(map_res)?)
        }
    }

    async fn properties(
        &self,
        ctx: &Context<'_>,
        id: Option<ID>,
        _query: Option<String>,
    ) -> Result<Vec<Property>> {
        if let Some(id) = id {
            Ok(vec![db(&ctx.into())
                .properties()
                .by_id(&id.parse::<PropertyId>()?)?
                .into()])
        } else {
            Ok(db(&ctx.into())
                .properties()
                .by_auth_id(ctx.data::<AuthId>()?)
                .and_then(map_res)?)
        }
    }

    async fn summary(
        &self,
        _ctx: &Context<'_>,
        _since: Option<DateTime>,
        _until: Option<DateTime>,
    ) -> Result<Summary> {
        Ok(piteo::get_summary().map(Summary::from)?)
    }

    async fn tenants(
        &self,
        ctx: &Context<'_>,
        id: Option<ID>,
        _query: Option<String>,
        _status: Option<TenantStatus>,
    ) -> Result<Vec<Tenant>> {
        let id = id.map(|id| id.parse::<TenantId>().unwrap_or_default());

        if let Some(id) = id {
            Ok(vec![db(&ctx.into()).tenants().by_id(&id)?.into()])
        } else {
            Ok(db(&ctx.into())
                .tenants()
                .by_auth_id(ctx.data::<AuthId>()?)
                .and_then(map_res)?)
        }
    }

    async fn leases(
        &self,
        ctx: &Context<'_>,
        _id: Option<ID>,
        _query: Option<String>,
    ) -> Result<Vec<Lease>> {
        Ok(db(&ctx.into())
            .leases()
            .by_auth_id(ctx.data::<AuthId>()?)
            .and_then(map_res)?)
    }

    async fn lenders(
        &self,
        ctx: &Context<'_>,
        id: Option<ID>,
        _query: Option<String>,
    ) -> Result<Vec<Lender>> {
        let id = id.map(|id| id.parse::<LenderId>().unwrap_or_default());

        if let Some(id) = id {
            Ok(vec![db(&ctx.into()).lenders().by_id(&id)?.0.into()])
        } else {
            Ok(db(&ctx.into())
                .lenders()
                .by_auth_id(ctx.data::<AuthId>()?)
                .and_then(map_res)?)
        }
    }

    async fn rents(
        &self,
        ctx: &Context<'_>,
        _since: DateTime,
        _until: DateTime,
    ) -> Result<Vec<Rent>> {
        Ok(db(&ctx.into())
            .rents()
            .by_auth_id(ctx.data::<AuthId>()?)
            .and_then(map_res)?)
    }

    async fn events(&self, ctx: &Context<'_>) -> Result<Vec<Event>> {
        Ok(db(&ctx.into())
            .events()
            .by_auth_id(ctx.data::<AuthId>()?)
            .and_then(map_res)?)
    }

    async fn transactions(&self, _ctx: &Context<'_>, _id: Option<ID>) -> Result<Vec<Payment>> {
        Ok(Vec::new())
    }

    async fn invoices(&self, _ctx: &Context<'_>) -> Result<Vec<Invoice>> {
        Ok(Vec::new())
    }

    async fn plans(&self, _ctx: &Context<'_>) -> Result<Vec<Plan>> {
        Ok(Vec::new())
    }

    async fn files(&self, _ctx: &Context<'_>) -> Result<Vec<File>> {
        Ok(Vec::new())
    }
}

// # Utils

pub(crate) fn map_res<T, U>(vec: Vec<T>) -> std::result::Result<Vec<U>, piteo::Error>
where
    T: Clone,
    U: From<T>,
{
    Ok(vec.iter().map(|item| item.clone().into()).collect())
}
