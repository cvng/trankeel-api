pub async fn codegen() {
    trankeel_graphql::write_schema("schema.graphql").unwrap();
    println!("💫 GraphQL schema printed.");
}
