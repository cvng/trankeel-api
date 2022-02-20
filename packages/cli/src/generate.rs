use trankeel::config::Config;

pub async fn generate(_config: Config) {
    trankeel_graphql::write_schema("schema.graphql").unwrap();

    println!("💫 GraphQL schema printed.");
}
