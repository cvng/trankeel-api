#![deny(clippy::disallowed_type)]

#[macro_use]
extern crate async_graphql;

mod interfaces;
mod objects;
mod payloads;
mod schema;
mod unions;

pub use crate::schema::build_schema;
pub use crate::schema::write_schema;
pub use crate::schema::Schema;
pub use async_graphql::extensions;
pub use async_graphql::http;
