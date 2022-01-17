use crate::routes;
#[cfg(debug_assertions)]
use crate::routes::debug_routes;
use crate::webhooks;
use rocket::routes;
use rocket::Build;
use rocket::Rocket;
use rocket_cors::CorsOptions;
use rocket_dyn_templates::Template;
use trankeel::config::Config;
use trankeel::Result;
use trankeel_graphql::extensions::ApolloTracing;

/// Build Trankeel web server.
///
/// https://rocket.rs
pub fn server(config: &Config) -> Result<Rocket<Build>> {
    let client = trankeel::init(config)?;

    let schema = trankeel_graphql::build_schema()
        .extension(ApolloTracing)
        .data(client.clone())
        .finish();

    let server = rocket::build()
        .attach(CorsOptions::default().to_cors()?)
        .attach(Template::fairing())
        .manage(client)
        .manage(schema)
        .mount(
            "/",
            routes![
                routes::graphql_playground,
                routes::graphql_request,
                webhooks::pdfmonkey_request
            ],
        );

    #[cfg(debug_assertions)]
    let server = mount_debug_routes(server);

    Ok(server)
}

#[cfg(debug_assertions)]
fn mount_debug_routes(server: Rocket<Build>) -> Rocket<Build> {
    server.mount("/debug", routes![debug_routes::preview_request])
}
