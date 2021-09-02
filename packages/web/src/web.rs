use std::env;
use std::process::Command;

fn main() {
    env::set_current_dir(&env::var("CARGO_MANIFEST_DIR").unwrap()).ok();
    Command::new("npx").arg("next").arg("dev").status().ok();
}
