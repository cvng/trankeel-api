//! Piteo.

mod guards;
mod routes;
mod server;

use color_eyre::Result;
use rocket::launch;

#[launch]
fn rocket() -> _ {
    color_eyre::install().ok();
    dotenv::dotenv().ok();
    piteo_graphql::write_schema().ok();
    server::server().unwrap()
}
