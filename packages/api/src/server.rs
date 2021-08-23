use crate::graphql;
use crate::routes::graphql_playground;
use crate::routes::graphql_query;
use crate::routes::graphql_request;
use crate::Result;
use rocket::routes;
use rocket::Build;
use rocket::Rocket;

/// Build Piteo Web server. https://rocket.rs
pub fn server() -> Result<Rocket<Build>> {
    let schema = graphql::build_schema()?;

    let server = rocket::build().manage(schema).mount(
        "/",
        routes![graphql_playground, graphql_query, graphql_request],
    );

    Ok(server)
}
