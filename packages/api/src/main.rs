//! Piteo.

mod guards;
mod routes;
mod server;
mod webhooks;

use color_eyre::Result;
use rocket::launch;

#[launch]
fn rocket() -> _ {
    color_eyre::install().ok();
    dotenv::dotenv().ok();
    piteo_graphql::write_schema("schema.graphql").ok();
    server::server().unwrap()
}
