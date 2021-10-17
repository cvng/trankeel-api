//! Trankeel.

mod guards;
mod routes;
mod server;
mod webhooks;

use rocket::launch;

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    trankeel_graphql::write_schema("schema.graphql").ok();
    server::server().unwrap()
}
