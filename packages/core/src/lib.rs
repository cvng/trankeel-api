#[macro_use]
extern crate diesel;

mod database;
mod schema;

pub mod address;
pub mod auth_id;
pub mod error;
pub mod person;
pub mod properties;

pub use crate::address::Address;
pub use crate::auth_id::AuthId;
pub use crate::database::build_connection_pool;
pub use crate::database::DbPool;
pub use crate::error::Error;
pub use crate::person::Person;
pub use crate::properties::Property;
