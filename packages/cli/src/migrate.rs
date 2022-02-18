use std::io;
use std::io::Write;
use std::process::Command;
use trankeel::config::Config;

pub async fn migrate(_config: Config) {
    let output = Command::new("diesel")
        .args(&["migration", "run"])
        .output()
        .unwrap();

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    println!("🌙 Database schema migrated.");
}
