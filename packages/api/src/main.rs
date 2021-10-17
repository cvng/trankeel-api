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
    let _guard = sentry();
    server::server().unwrap()
}

fn sentry() -> sentry::ClientInitGuard {
    sentry::init((
        std::env::var("SENTRY_DSN").expect("SENTRY_DSN"),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ))
}
