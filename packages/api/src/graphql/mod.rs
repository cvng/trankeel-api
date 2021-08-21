mod person;
mod property;
mod query;

use crate::graphql::query::Query;
use async_graphql::EmptyMutation;
use async_graphql::EmptySubscription;
use async_graphql::Schema;
use piteo_core::build_connection_pool;
use piteo_core::AuthId;
use piteo_core::Context;
use piteo_core::DbPool;
use std::env;

/// Piteo GraphQL schema.
pub type PiteoSchema = Schema<Query, EmptyMutation, EmptySubscription>;

/// Build Piteo GraphQL schema. https://async-graphql.github.io
pub fn build_schema() -> PiteoSchema {
    dotenv::dotenv().ok();

    let context = Context::new(db_pool_from_env(), auth_id_from_env());

    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(context)
        .finish()
}

/// Try creating database pool from env in debug.
fn db_pool_from_env() -> DbPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    build_connection_pool(&database_url)
}

/// Try reading authentication ID from env in debug.
fn auth_id_from_env() -> AuthId {
    let auth_id = env::var("FIREBASE_ADMIN_USER_ID").expect("FIREBASE_ADMIN_USER_ID must be set");
    AuthId::new(&auth_id)
}
