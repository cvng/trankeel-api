#[macro_use]
extern crate diesel;

mod database;
mod schema;

pub mod auth;
pub mod error;
pub mod properties;

pub use crate::auth::AuthId;
pub use crate::auth::Person;
pub use crate::database::build_connection_pool;
pub use crate::database::DbPool;
pub use crate::error::eyre::Result;
pub use crate::properties::Property;
