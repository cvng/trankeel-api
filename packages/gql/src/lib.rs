mod enums;
mod mutation;
mod objects;
mod query;
mod scalars;
mod unions;

pub use async_graphql::http;

use crate::mutation::Mutation;
use crate::query::Query;
use async_graphql::extensions::ApolloTracing;
use async_graphql::EmptySubscription;
use async_graphql::Schema;
use piteo_core::error::Context;
use piteo_core::DbPool;
use std::env;

type Result<T> = std::result::Result<T, piteo_core::Error>;

/// Piteo GraphQL schema.
pub type PiteoSchema = Schema<Query, Mutation, EmptySubscription>;

/// Build Piteo GraphQL schema. https://async-graphql.github.io
pub fn build_schema() -> Result<PiteoSchema> {
    let db_pool = db_pool_from_env()?;

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .extension(ApolloTracing)
        .data(db_pool)
        .finish();

    Ok(schema)
}

/// Build database pool from env.
fn db_pool_from_env() -> Result<DbPool> {
    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set.")?;

    piteo_core::build_connection_pool(&database_url)
}
