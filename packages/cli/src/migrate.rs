use std::io;
use std::io::Write;
use std::process::Command;

pub async fn migrate() {
    let output = Command::new("diesel")
        .args(&["migration", "run"])
        .output()
        .unwrap();

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
