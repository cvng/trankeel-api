use crate::graphql::person::Person;
use async_graphql::Result;
use piteo_core as core;

/// Query object.
pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn viewer(&self, auth_id: String) -> Result<Person> {
        match core::auth::first_by_auth_id(&auth_id) {
            Ok(person) => Ok(person.into()),
            Err(err) => Err(map_err(err)),
        }
    }
}

// # Utils

fn map_err(err: core::Error) -> async_graphql::Error {
    async_graphql::Error::new(err.to_string())
}
