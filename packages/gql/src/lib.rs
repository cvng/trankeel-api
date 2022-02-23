#![deny(clippy::disallowed_type)]

mod interfaces;
mod objects;
mod payloads;
mod schema;
mod unions;
mod unions2;

pub use crate::schema::build_schema;
pub use crate::schema::write_schema;
pub use crate::schema::Schema;
pub use async_graphql::extensions;
pub use async_graphql::http;
pub use async_graphql::Data;
pub use async_graphql::Response;
pub use async_graphql::ServerError;
