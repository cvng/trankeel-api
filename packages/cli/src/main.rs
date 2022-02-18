mod generate;
mod migrate;
mod seed;

#[tokio::main]
async fn main() {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "info");
    }

    tracing_subscriber::fmt::init();

    #[cfg(feature = "dotenv")]
    dotenv::dotenv().unwrap();

    let config = trankeel::config::config();

    let command = std::env::args().collect::<Vec<_>>().get(1).cloned();

    match command.as_deref() {
        Some("generate") => generate::generate(config).await,
        Some("migrate") => migrate::migrate(config).await,
        Some("seed") => seed::seed(config).await,
        _ => eprintln!("error: invalid command: `{:?}`", command),
    }
}
