//! Trankeel.

mod guards;
mod routes;
mod server;
mod webhooks;

#[rocket::launch]
async fn rocket() -> _ {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    #[cfg(feature = "dotenv")]
    dotenv::dotenv().unwrap();

    let config = trankeel::config::config();

    #[cfg(not(feature = "release"))]
    trankeel_graphql::write_schema(config.graphql.get("schema").unwrap()).ok();

    #[cfg(feature = "sentry")]
    let _guard = sentry::init((
        config.sentry_dsn.clone(),
        sentry::ClientOptions {
            release: sentry::release_name!(),
            ..Default::default()
        },
    ));

    server::server(config).await.unwrap()
}
