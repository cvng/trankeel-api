//! Trankeel.

mod guards;
mod routes;
mod server;
mod webhooks;

#[rocket::launch]
fn rocket() -> _ {
    #[cfg(debug_assertions)]
    dotenv::dotenv().unwrap();

    let config = trankeel::config::config();

    trankeel_graphql::write_schema("schema.graphql").ok();

    #[cfg(feature = "sentry")]
    let _guard = init_sentry(&config);

    server::server(&config).unwrap()
}

#[cfg(feature = "sentry")]
fn init_sentry(config: &trankeel::config::Config) -> sentry::ClientInitGuard {
    sentry::init((
        config.sentry_dsn.clone(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ))
}
