use crate::guards::AuthGuard;
use async_graphql_rocket::GraphQLRequest;
use async_graphql_rocket::GraphQLResponse;
use rocket::get;
use rocket::post;
use rocket::response::content;
use rocket::State;
use trankeel_graphql::http::playground_source;
use trankeel_graphql::http::GraphQLPlaygroundConfig;
use trankeel_graphql::Schema;

#[get("/graphql")]
pub fn graphql_playground() -> content::Html<String> {
    content::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(
    schema: &State<Schema>,
    request: GraphQLRequest,
    auth: AuthGuard,
) -> GraphQLResponse {
    match auth.inner() {
        Some(auth_id) => request.data(auth_id).execute(schema).await,
        None => request.execute(schema).await,
    }
}
