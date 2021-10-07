use crate::guards::AuthGuard;
use async_graphql_rocket::Request;
use async_graphql_rocket::Response;
use piteo_graphql::http::playground_source;
use piteo_graphql::http::GraphQLPlaygroundConfig;
use piteo_graphql::Schema;
use rocket::get;
use rocket::post;
use rocket::response::content;
use rocket::State;

#[get("/graphql")]
pub fn graphql_playground() -> content::Html<String> {
    content::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(
    schema: &State<Schema>,
    request: Request,
    auth: AuthGuard,
) -> Response {
    match auth.inner() {
        Some(auth_id) => request.data(auth_id).execute(schema).await,
        None => request.execute(schema).await,
    }
}
