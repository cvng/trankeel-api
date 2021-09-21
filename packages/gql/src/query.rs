use crate::objects::Event;
use crate::objects::File;
use crate::objects::Invoice;
use crate::objects::Lease;
use crate::objects::Lender;
use crate::objects::Person;
use crate::objects::Plan;
use crate::objects::Property;
use crate::objects::Rent;
use crate::objects::Summary;
use crate::objects::Tenant;
use crate::objects::Transaction;
use crate::scalars::DateTime;
use async_graphql::Context;
use async_graphql::Result;
use async_graphql::ID;
use piteo::database::Db;
use piteo::db;
use piteo::reports;
use piteo::AuthId;
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
        Ok(db(db_pool.clone())
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
            Ok(vec![db(db_pool.clone()).properties().by_id(&id)?.into()])
        } else {
            Ok(db(db_pool.clone())
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
            Ok(vec![db(db_pool.clone()).tenants().by_id(&id)?.into()])
        } else {
            Ok(db(db_pool.clone())
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

        Ok(db(db_pool.clone())
            .leases()
            .by_auth_id(auth_id)
            .and_then(map_res)?)
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
            Ok(vec![db(db_pool.clone())
                .lenders()
                .by_id(&id)?
                .lender()
                .into()])
        } else {
            Ok(db(db_pool.clone())
                .lenders()
                .by_auth_id(auth_id)?
                .iter()
                .map(|lender_identity| lender_identity.lender().into())
                .collect::<Vec<_>>())
        }
    }

    async fn rents(
        &self,
        _ctx: &Context<'_>,
        _since: DateTime,
        _until: DateTime,
    ) -> Result<Vec<Rent>> {
        Ok(Vec::new())
    }

    async fn events(&self, _ctx: &Context<'_>) -> Result<Vec<Event>> {
        Ok(Vec::new())
    }

    async fn transactions(&self, _ctx: &Context<'_>, _id: Option<ID>) -> Result<Vec<Transaction>> {
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
