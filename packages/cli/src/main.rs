mod codegen;
mod migrate;
mod seed;

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv::dotenv().unwrap();

    let args: Vec<String> = std::env::args().collect();
    let command = args[1].as_str();

    match command {
        "codegen" => codegen::codegen().await,
        "migrate" => migrate::migrate().await,
        "seed" => seed::seed().await,
        _ => eprintln!("error: invalid command: `{command}`"),
    }
    }
