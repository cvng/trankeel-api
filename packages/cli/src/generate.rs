pub async fn generate() {
    trankeel_graphql::write_schema("schema.graphql").unwrap();

    log::info!("💫 GraphQL schema printed.");
}
