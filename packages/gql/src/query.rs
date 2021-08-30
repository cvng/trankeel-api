use crate::enums::TenantStatus;
use crate::objects::File;
use crate::objects::Invoice;
use crate::objects::Lease;
use crate::objects::Lender;
use crate::objects::Person;
use crate::objects::Plan;
use crate::objects::Property;
use crate::objects::Summary;
use crate::objects::Tenant;
use crate::objects::Transaction;
use crate::scalars::DateTime;
use async_graphql::Context;
use async_graphql::Result;
use async_graphql::ID;
use piteo_core::auth;
use piteo_core::leases;
use piteo_core::properties;
use piteo_core::reports;
use piteo_core::tenants;
use piteo_core::AuthId;
use piteo_core::DbPool;
use piteo_core::Id;

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn viewer(&self, ctx: &Context<'_>) -> Result<Person> {
        let conn = ctx.data::<DbPool>()?.get()?;
        let auth_id = ctx.data::<AuthId>()?;

        Ok(auth::find(&conn, auth_id).map(Person::from)?)
    }

    async fn properties(
        &self,
        ctx: &Context<'_>,
        id: Option<ID>,
        _query: Option<String>,
    ) -> Result<Vec<Property>> {
        let conn = ctx.data::<DbPool>()?.get()?;
        let auth_id = ctx.data::<AuthId>()?;
        let id = id.map(|id| Id::parse_str(&id).unwrap_or_default());

        Ok(properties::all_properties(&conn, auth_id, id).and_then(map_res)?)
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
        let conn = ctx.data::<DbPool>()?.get()?;
        let auth_id = ctx.data::<AuthId>()?;
        let id = id.map(|id| Id::parse_str(&id).unwrap_or_default());

        Ok(tenants::all_tenants(&conn, auth_id, id).and_then(map_res)?)
    }

    async fn leases(
        &self,
        ctx: &Context<'_>,
        _id: Option<ID>,
        _query: Option<String>,
    ) -> Result<Vec<Lease>> {
        let conn = ctx.data::<DbPool>()?.get()?;
        let auth_id = ctx.data::<AuthId>()?;

        Ok(leases::all_leases(&conn, auth_id).and_then(map_res)?)
    }

    async fn lenders(
        &self,
        ctx: &Context<'_>,
        id: Option<ID>,
        _query: Option<String>,
    ) -> Result<Vec<Lender>> {
        let conn = ctx.data::<DbPool>()?.get()?;
        let auth_id = ctx.data::<AuthId>()?;
        let id = id.map(|id| Id::parse_str(&id).unwrap_or_default());

        Ok(properties::all_lenders(&conn, auth_id, id).and_then(map_res)?)
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

fn map_res<T, U>(vec: Vec<T>) -> std::result::Result<Vec<U>, piteo_core::Error>
where
    T: Clone,
    U: From<T>,
{
    Ok(vec.iter().map(|item| item.clone().into()).collect())
}
