use crate::objects::Lease;
use crate::objects::Person;
use crate::objects::Property;
use crate::objects::Tenant;
use async_graphql::Context;
use async_graphql::Result;
use piteo_core::auth;
use piteo_core::leases;
use piteo_core::properties;
use piteo_core::tenants;
use piteo_core::AuthId;
use piteo_core::DbPool;

pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn viewer(&self, ctx: &Context<'_>) -> Result<Person> {
        match auth::first_by_auth_id(&ctx.data::<DbPool>()?.get()?, ctx.data::<AuthId>()?) {
            Ok(person) => Ok(person.into()),
            Err(err) => Err(map_err(err)),
        }
    }

    async fn properties(&self, ctx: &Context<'_>) -> Result<Vec<Property>> {
        match properties::load_by_auth_id(&ctx.data::<DbPool>()?.get()?, ctx.data::<AuthId>()?) {
            Ok(properties) => Ok(map_vec(properties)),
            Err(err) => Err(map_err(err)),
        }
    }

    async fn tenants(&self, ctx: &Context<'_>) -> Result<Vec<Tenant>> {
        match tenants::load_by_auth_id(&ctx.data::<DbPool>()?.get()?, ctx.data::<AuthId>()?) {
            Ok(tenants) => Ok(map_vec(tenants)),
            Err(err) => Err(map_err(err)),
        }
    }

    async fn leases(&self, ctx: &Context<'_>) -> Result<Vec<Lease>> {
        match leases::load_by_auth_id(&ctx.data::<DbPool>()?.get()?, ctx.data::<AuthId>()?) {
            Ok(leases) => Ok(map_vec(leases)),
            Err(err) => Err(map_err(err)),
        }
    }
}

// # Utils

fn map_vec<T: Clone, U: From<T>>(vec: Vec<T>) -> Vec<U> {
    vec.iter().map(|item| item.clone().into()).collect()
}

fn map_err(err: piteo_core::Error) -> async_graphql::Error {
    async_graphql::Error::new(err.to_string())
}
