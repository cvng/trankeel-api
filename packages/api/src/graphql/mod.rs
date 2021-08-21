mod person;
mod property;
mod query;

use crate::graphql::query::Query;
use async_graphql::EmptyMutation;
use async_graphql::EmptySubscription;
use async_graphql::Schema;
use piteo_core::auth_id_fallback_from_env;

/// Piteo GraphQL schema.
pub type PiteoSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// Build Piteo GraphQL schema. https://async-graphql.github.io
pub fn build_schema() -> PiteoSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(auth_id_fallback_from_env())
        .finish()
}
