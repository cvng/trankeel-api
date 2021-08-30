use crate::enums::TenantStatus;
use crate::objects::Lease;
use crate::objects::Lender;
use crate::objects::Person;
use crate::objects::Property;
use crate::objects::Summary;
use crate::objects::Tenant;
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
        match auth::first_by_auth_id(&ctx.data::<DbPool>()?.get()?, ctx.data::<AuthId>()?) {
            Ok(person) => Ok(person.into()),
            Err(err) => Err(map_err(err)),
        }
    }

    async fn properties(
        &self,
        ctx: &Context<'_>,
        id: Option<ID>,
        _query: Option<String>,
    ) -> Result<Vec<Property>> {
        match properties::load_by_auth_id(
            &ctx.data::<DbPool>()?.get()?,
            ctx.data::<AuthId>()?,
            id.map(|id| Id::parse_str(&id).unwrap_or_default()),
        ) {
            Ok(properties) => Ok(map_vec(properties)),
            Err(err) => Err(map_err(err)),
        }
    }

    async fn summary(
        &self,
        _ctx: &Context<'_>,
        _since: Option<DateTime>,
        _until: Option<DateTime>,
    ) -> Result<Summary> {
        match reports::get_summary() {
            Ok(summary) => Ok(summary.into()),
            Err(err) => Err(map_err(err)),
        }
    }

    async fn tenants(
        &self,
        ctx: &Context<'_>,
        id: Option<ID>,
        _query: Option<String>,
        _status: Option<TenantStatus>,
    ) -> Result<Vec<Tenant>> {
        match tenants::load_by_auth_id(
            &ctx.data::<DbPool>()?.get()?,
            ctx.data::<AuthId>()?,
            id.map(|id| Id::parse_str(&id).unwrap_or_default()),
        ) {
            Ok(tenants) => Ok(map_vec(tenants)),
            Err(err) => Err(map_err(err)),
        }
    }

    async fn leases(
        &self,
        ctx: &Context<'_>,
        _id: Option<ID>,
        _query: Option<String>,
    ) -> Result<Vec<Lease>> {
        match leases::load_by_auth_id(&ctx.data::<DbPool>()?.get()?, ctx.data::<AuthId>()?) {
            Ok(leases) => Ok(map_vec(leases)),
            Err(err) => Err(map_err(err)),
        }
    }

    async fn lenders(
        &self,
        ctx: &Context<'_>,
        id: Option<ID>,
        _query: Option<String>,
    ) -> Result<Vec<Lender>> {
        match properties::load_lenders_by_auth_id(
            &ctx.data::<DbPool>()?.get()?,
            ctx.data::<AuthId>()?,
            id.map(|id| Id::parse_str(&id).unwrap_or_default()),
        ) {
            Ok(leases) => Ok(map_vec(leases)),
            Err(err) => Err(map_err(err)),
        }
    }
}

// # Utils

fn map_vec<T: Clone, U: From<T>>(vec: Vec<T>) -> Vec<U> {
    vec.iter().map(|item| item.clone().into()).collect()
}

pub fn map_err(err: piteo_core::Error) -> async_graphql::Error {
    async_graphql::Error::new(err.to_string())
}
