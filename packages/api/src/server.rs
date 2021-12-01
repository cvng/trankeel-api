use crate::routes::graphql_playground;
use crate::routes::graphql_request;
use crate::webhooks::pdfmonkey_request;
use rocket::routes;
use rocket::Build;
use rocket::Rocket;
use trankeel::Result;
use trankeel_graphql::extensions::ApolloTracing;

/// Build Trankeel Web server. https://rocket.rs
pub fn server() -> Result<Rocket<Build>> {
    let cors = rocket_cors::CorsOptions::default().to_cors()?;

    let client = trankeel::init()?;

    let schema = trankeel_graphql::build_schema()
        .extension(ApolloTracing)
        .data(client)
        .finish();

    let server = rocket::build()
        .manage(schema)
        .mount(
            "/",
            routes![graphql_playground, graphql_request, pdfmonkey_request],
        )
        .attach(cors);

    Ok(server)
}
