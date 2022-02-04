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
use trankeel::PublicError;

pub type Schema = async_graphql::Schema<Query, Mutation, EmptySubscription>;

/// Build the GraphQL schema.
///
/// https://async-graphql.github.io
pub fn build_schema() -> SchemaBuilder<Query, Mutation, EmptySubscription> {
    async_graphql::Schema::build(Query, Mutation, EmptySubscription)
        .register_output_type::<PublicError>()
        .register_output_type::<PersonInterface>()
        .register_output_type::<LegalIdentityInterface>()
}

/// Print the GraphQL schema in SDL format.
///
/// https://async-graphql.github.io/async-graphql/en/sdl_export.html
pub fn write_schema<P>(path: P) -> io::Result<()>
where
    P: AsRef<Path>,
{
    let schema = build_schema().finish();

    let mut file = File::create(path)?;

    file.write_all(schema.sdl().as_bytes())
}
