use crate::graphql;
use crate::routes::graphql_playground;
use crate::routes::graphql_query;
use crate::routes::graphql_request;
use rocket::routes;
use rocket::Build;
use rocket::Rocket;

/// Build Piteo Web server. https://rocket.rs
pub fn build_rocket() -> Rocket<Build> {
    let schema = graphql::build_schema();

    rocket::build().manage(schema).mount(
        "/",
        routes![graphql_playground, graphql_query, graphql_request],
    )
}
