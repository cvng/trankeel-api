use crate::graphql::person::Person;
use crate::graphql::property::Property;
use async_graphql::Context;
use async_graphql::Result;
use piteo_core::auth;
use piteo_core::properties;
use piteo_core::AuthId;
use piteo_core::DbPool;

/// Query object.
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
}

// # Utils

fn map_vec<T: Clone, U: From<T>>(vec: Vec<T>) -> Vec<U> {
    vec.iter().map(|item| item.clone().into()).collect()
}

fn map_err(err: piteo_core::Error) -> async_graphql::Error {
    async_graphql::Error::new(err.to_string())
}
