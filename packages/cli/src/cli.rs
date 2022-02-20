mod generate;
mod migrate;
mod seed;

#[tokio::main]
async fn main() {
    #[cfg(feature = "dotenv")]
    dotenv::dotenv().unwrap();

    tracing_subscriber::fmt::init();

    let config = trankeel::config::config();

    let command = std::env::args().collect::<Vec<_>>().get(1).cloned();

    match command.as_deref() {
        Some("seed") => seed::seed(config).await,
        Some("migrate") => migrate::migrate(config).await,
        Some("generate") => generate::generate(config).await,
        _ => eprintln!("error: invalid command: `{:?}`", command),
    }
}
