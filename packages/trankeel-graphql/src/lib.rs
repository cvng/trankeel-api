#![deny(clippy::disallowed_type)]

#[macro_use]
extern crate async_graphql;

mod interfaces;
mod objects;
mod payloads;
mod root;
mod unions;

pub use crate::root::build_schema;
pub use crate::root::write_schema;
pub use crate::root::Schema;
pub use async_graphql::http;

fn wip() -> async_graphql::Error {
    async_graphql::Error::new("wip!()")
}
