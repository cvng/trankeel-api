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
    #[cfg(not(debug_assertions))]
    let _guard = sentry();
    server::server().unwrap()
}

#[allow(dead_code)]
fn sentry() -> sentry::ClientInitGuard {
    sentry::init((
        std::env::var("SENTRY_DSN").expect("SENTRY_DSN"),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ))
}
