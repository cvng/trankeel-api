use crate::interfaces::LegalIdentityInterface;
use crate::interfaces::PersonInterface;
use crate::objects::Mutation;
use crate::objects::Query;
use async_graphql::extensions::ApolloTracing;
use async_graphql::EmptySubscription;
use std::fs::File;
use std::io::Write;
use std::result;

pub type Result<T> = result::Result<T, trankeel::Error>;

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

/// Build the GraphQL schema. https://async-graphql.github.io
pub fn build_schema() -> Result<Schema> {
    let client = trankeel::init()?;

    let schema = async_graphql::Schema::build(Query, Mutation, EmptySubscription)
        .register_output_type::<PersonInterface>()
        .register_output_type::<LegalIdentityInterface>()
        .extension(ApolloTracing)
        .data(client)
        .finish();

    Ok(schema)
}

/// Print the GraphQL schema in SDL format. https://async-graphql.github.io/async-graphql/en/sdl_export.html
pub fn write_schema(path: &str) -> Result<File> {
    let schema = build_schema()?;

    let mut file = File::create(path)?;
    file.write_all(schema.sdl().as_bytes())?;

    Ok(file)
}
