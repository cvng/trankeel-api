//! Trankeel.

mod guards;
mod routes;
mod server;
mod webhooks;

#[tokio::main]
async fn main() {
    #[cfg(feature = "dotenv")]
    dotenv::dotenv().unwrap();

    tracing_subscriber::fmt::init();

    let config = trankeel::config::config();

    #[cfg(not(feature = "release"))]
    trankeel_graphql::write_schema("schema.graphql").unwrap();

    #[cfg(feature = "sentry")]
    let _guard = sentry::init(config.sentry_dsn.clone());

    let app = server::app(config.clone()).await.unwrap();

    server::server(config).run(app).await.unwrap()
}
