//! Piteo.

mod graphql;
mod routes;
mod server;

use rocket::launch;

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    server::build_rocket()
}
