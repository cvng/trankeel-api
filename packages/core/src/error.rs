pub use eyre;
pub use eyre::Context;
pub use eyre::Error;

pub fn no(path: &'static str) -> Error {
    Error::msg(format!("Missing {}", path))
}
