use crate::guards::Token;
use async_graphql::http::playground_source;
use async_graphql::http::GraphQLPlaygroundConfig;
use async_graphql_rocket::Query;
use async_graphql_rocket::Request;
use async_graphql_rocket::Response;
use piteo_graphql::PiteoSchema;
use rocket::get;
use rocket::post;
use rocket::response::content;
use rocket::State;

#[get("/graphql")]
pub fn graphql_playground() -> content::Html<String> {
    content::Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}

#[get("/graphql?<query..>")]
pub async fn graphql_query(schema: &State<PiteoSchema>, query: Query, _token: Token) -> Response {
    query.execute(schema).await
}

#[post("/graphql", data = "<request>", format = "application/json")]
pub async fn graphql_request(
    schema: &State<PiteoSchema>,
    request: Request,
    token: Token,
) -> Response {
    let request = request.data(token.0);
    request.execute(schema).await
}
