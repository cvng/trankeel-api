use crate::routes;
#[cfg(debug_assertions)]
use crate::routes::dev_routes;
use crate::webhooks;
use rocket::routes;
use rocket::Build;
use rocket::Rocket;
use rocket_cors::CorsOptions;
use rocket_dyn_templates::Template;
use trankeel::Result;
use trankeel_graphql::extensions::ApolloTracing;

/// Build Trankeel Web server.
///
/// https://rocket.rs
pub fn server() -> Result<Rocket<Build>> {
    let client = trankeel::init()?;

    let schema = trankeel_graphql::build_schema()
        .extension(ApolloTracing)
        .data(client.clone())
        .finish();

    let server = rocket::build()
        .manage(client)
        .manage(schema)
        .mount(
            "/",
            routes![
                routes::graphql_playground,
                routes::graphql_request,
                webhooks::pdfmonkey_request
            ],
        )
        .attach(CorsOptions::default().to_cors()?)
        .attach(Template::fairing());

    #[cfg(debug_assertions)]
    let server = mount_dev_routes(server);

    Ok(server)
}

#[cfg(debug_assertions)]
fn mount_dev_routes(server: Rocket<Build>) -> Rocket<Build> {
    server.mount("/dev", routes![dev_routes::preview_request])
}
