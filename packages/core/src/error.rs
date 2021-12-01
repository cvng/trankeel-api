pub use eyre;
pub use eyre::Context;
pub use eyre::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub fn no(path: &str) -> Error {
    Error::msg(format!("Missing {}", path))
}
