mod interfaces;
mod mutation;
mod objects;
mod payloads;
mod query;
mod unions;

pub use async_graphql::http;

use crate::interfaces::LegalIdentityInterface;
use crate::interfaces::PersonInterface;
use crate::mutation::Mutation;
use crate::query::Query;
use async_graphql::extensions::ApolloTracing;
use async_graphql::EmptySubscription;
use async_graphql::Schema;
use std::fs::File;
use std::io::Write;

type Result<T> = std::result::Result<T, piteo::Error>;

/// Piteo GraphQL schema.
pub type PiteoSchema = Schema<Query, Mutation, EmptySubscription>;

/// Build Piteo GraphQL schema. https://async-graphql.github.io
pub fn build_schema() -> Result<PiteoSchema> {
    let client = piteo::init();

    let schema = Schema::build(Query, Mutation, EmptySubscription)
        .register_type::<PersonInterface>()
        .register_type::<LegalIdentityInterface>()
        .extension(ApolloTracing)
        .data(client)
        .finish();

    Ok(schema)
}

/// Print the schema in SDL format. https://async-graphql.github.io/async-graphql/en/sdl_export.html
pub fn write_schema(path: &str) -> Result<File> {
    let schema = build_schema()?;

    let mut file = File::create(path)?;
    file.write_all(schema.sdl().as_bytes())?;

    Ok(file)
}

fn wip() -> async_graphql::Error {
    async_graphql::Error::new("wip!()")
}
