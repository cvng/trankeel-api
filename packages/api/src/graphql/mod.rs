mod person;
mod property;
mod query;

use crate::graphql::query::Query;
use crate::Result;
use async_graphql::extensions::ApolloTracing;
use async_graphql::EmptyMutation;
use async_graphql::EmptySubscription;
use async_graphql::Schema;
use color_eyre::eyre::Context;
use piteo_core::build_connection_pool;
use piteo_core::DbPool;
use std::env;

/// Piteo GraphQL schema.
pub type PiteoSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// Build Piteo GraphQL schema. https://async-graphql.github.io
pub fn build_schema() -> Result<PiteoSchema> {
    let db_pool = db_pool_from_env()?;

    let schema = Schema::build(Query, EmptyMutation, EmptySubscription)
        .extension(ApolloTracing)
        .data(db_pool)
        .finish();

    Ok(schema)
}

/// Build database pool from env.
fn db_pool_from_env() -> Result<DbPool> {
    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set.")?;

    build_connection_pool(&database_url)
}
