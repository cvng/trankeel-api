//! Piteo.

mod graphql;
mod guards;
mod routes;
mod server;

use color_eyre::Result;
use rocket::launch;

#[launch]
fn rocket() -> _ {
    color_eyre::install().ok();
    dotenv::dotenv().ok();
    server::server().unwrap()
}
