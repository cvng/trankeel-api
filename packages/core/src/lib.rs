#[macro_use]
extern crate diesel;

mod database;
mod schema;

pub mod auth;
pub mod properties;

pub use database::build_connection_pool;
pub use database::DbPool;

pub type Error = diesel::result::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Context
pub struct Context {
    auth_id: AuthId,
    db_pool: DbPool,
}

impl Context {
    pub fn new(db_pool: DbPool, auth_id: AuthId) -> Self {
        Self { auth_id, db_pool }
    }
}

/// Authentication ID
pub struct AuthId(String);

impl AuthId {
    pub fn new(auth_id: &str) -> Self {
        Self(auth_id.into())
    }
}
