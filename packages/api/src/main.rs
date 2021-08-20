//! Piteo.

mod graphql;
mod routes;
mod server;

use rocket::launch;

#[launch]
fn rocket() -> _ {
    server::build_rocket()
}
