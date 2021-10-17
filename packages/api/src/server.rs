use crate::routes::graphql_playground;
use crate::routes::graphql_request;
use crate::webhooks::pdfmonkey_request;
use crate::Result;
use rocket::routes;
use rocket::Build;
use rocket::Rocket;

/// Build Trankeel Web server. https://rocket.rs
pub fn server() -> Result<Rocket<Build>> {
    let cors = rocket_cors::CorsOptions::default().to_cors()?;

    let schema = trankeel_graphql::build_schema()?;

    let server = rocket::build()
        .manage(schema)
        .mount(
            "/",
            routes![graphql_playground, graphql_request, pdfmonkey_request],
        )
        .attach(cors);

    Ok(server)
}
