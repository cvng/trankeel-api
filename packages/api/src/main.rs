//! Trankeel.

mod guards;
mod routes;
mod server;
mod webhooks;

#[rocket::launch]
async fn rocket() -> _ {
    init_logger();

    #[cfg(debug_assertions)]
    dotenv::dotenv().unwrap();

    let config = trankeel::config::config();

    trankeel_graphql::write_schema("schema.graphql").ok();

    #[cfg(feature = "sentry")]
    let _guard = init_sentry(&config);

    server::server(config).await.unwrap()
}

fn init_logger() {
    use std::io::Write;

    env_logger::builder()
        .format(|buf, record| writeln!(buf, "{} {}", record.level(), record.args()))
        .filter(None, log::LevelFilter::Info)
        .init()
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
