use crate::interfaces::LegalIdentityInterface;
use crate::interfaces::PersonInterface;
use crate::objects::Mutation;
use crate::objects::Query;
use async_graphql::EmptySubscription;
use async_graphql::SchemaBuilder;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

/// Build the GraphQL schema.
///
/// https://async-graphql.github.io
pub fn build_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    async_graphql::Schema::build(Query, Mutation, EmptySubscription)
        .register_output_type::<PersonInterface>()
        .register_output_type::<LegalIdentityInterface>()
}

/// Print the GraphQL schema in SDL format.
///
/// https://async-graphql.github.io/async-graphql/en/sdl_export.html
pub fn write_schema<P: AsRef<Path>>(path: P) -> io::Result<()> {
    let schema = build_schema().finish();

    let mut file = File::create(path)?;

    file.write_all(schema.sdl().as_bytes())
}
