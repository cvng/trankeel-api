//! Piteo.

mod graphql;
mod guards;
mod routes;
mod server;

use rocket::launch;

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    server::server().unwrap()
}
