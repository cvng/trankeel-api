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
use piteo::database::Db;
use piteo::db;
use piteo::reports;
use piteo::AuthId;
use piteo::DateTime;
use piteo::DbPool;
use piteo::LenderId;
use piteo::PropertyId;
use piteo::TenantId;
use piteo::TenantStatus;

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn viewer(&self, ctx: &Context<'_>) -> Result<Person> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;
        Ok(db(db_pool)
            .persons()
            .by_auth_id(auth_id)
            .map(Person::from)?)
    }

    async fn properties(
        &self,
        ctx: &Context<'_>,
        id: Option<ID>,
        _query: Option<String>,
    ) -> Result<Vec<Property>> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;
        let id = id.map(|id| PropertyId::parse_str(&id).unwrap_or_default());

        if let Some(id) = id {
            Ok(vec![db(db_pool).properties().by_id(&id)?.into()])
        } else {
            Ok(db(db_pool)
                .properties()
                .by_auth_id(auth_id)
                .and_then(map_res)?)
        }
    }

    async fn summary(
        &self,
        _ctx: &Context<'_>,
        _since: Option<DateTime>,
        _until: Option<DateTime>,
    ) -> Result<Summary> {
        Ok(reports::get_summary().map(Summary::from)?)
    }

    async fn tenants(
        &self,
        ctx: &Context<'_>,
        id: Option<ID>,
        _query: Option<String>,
        _status: Option<TenantStatus>,
    ) -> Result<Vec<Tenant>> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;
        let id = id.map(|id| TenantId::parse_str(&id).unwrap_or_default());

        if let Some(id) = id {
            Ok(vec![db(db_pool).tenants().by_id(&id)?.into()])
        } else {
            Ok(db(db_pool)
                .tenants()
                .by_auth_id(auth_id)
                .and_then(map_res)?)
        }
    }

    async fn leases(
        &self,
        ctx: &Context<'_>,
        _id: Option<ID>,
        _query: Option<String>,
    ) -> Result<Vec<Lease>> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;

        Ok(db(db_pool).leases().by_auth_id(auth_id).and_then(map_res)?)
    }

    async fn lenders(
        &self,
        ctx: &Context<'_>,
        id: Option<ID>,
        _query: Option<String>,
    ) -> Result<Vec<Lender>> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;
        let id = id.map(|id| LenderId::parse_str(&id).unwrap_or_default());

        if let Some(id) = id {
            Ok(vec![db(db_pool).lenders().by_id(&id)?.lender().into()])
        } else {
            Ok(db(db_pool)
                .lenders()
                .by_auth_id(auth_id)
                .and_then(map_res)?)
        }
    }

    async fn rents(
        &self,
        ctx: &Context<'_>,
        _since: DateTime,
        _until: DateTime,
    ) -> Result<Vec<Rent>> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;

        Ok(db(db_pool).rents().by_auth_id(auth_id).and_then(map_res)?)
    }

    async fn events(&self, ctx: &Context<'_>) -> Result<Vec<Event>> {
        let db_pool = ctx.data::<DbPool>()?;
        let auth_id = ctx.data::<AuthId>()?;

        Ok(db(db_pool).events().by_auth_id(auth_id).and_then(map_res)?)
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

pub(crate) fn map_res<T, U>(vec: Vec<T>) -> std::result::Result<Vec<U>, piteo::error::Error>
where
    T: Clone,
    U: From<T>,
{
    Ok(vec.iter().map(|item| item.clone().into()).collect())
}
