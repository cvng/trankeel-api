use trankeel::config::Config;

pub async fn generate(config: Config) {
    trankeel_graphql::write_schema(config.graphql.get("schema").unwrap()).unwrap();

    println!("💫 GraphQL schema printed.");
}
