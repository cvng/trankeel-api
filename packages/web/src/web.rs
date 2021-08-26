use std::process::Command;

fn main() {
  Command::new("npx").arg("next").arg("start").status().ok();
}
