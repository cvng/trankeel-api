use crate::guards::AuthGuard;
use async_graphql_rocket::Request;
use async_graphql_rocket::Response;
use piteo_graphql::http::playground_source;
use piteo_graphql::http::GraphQLPlaygroundConfig;
use piteo_graphql::PiteoSchema;
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
    schema: &State<PiteoSchema>,
    request: Request,
    auth: AuthGuard,
) -> Response {
    let request = request.data(auth.inner());
    request.execute(schema).await
}
