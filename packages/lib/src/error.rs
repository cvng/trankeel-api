pub use trankeel_core::error::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub(crate) fn no(path: &str) -> Error {
    Error::msg(format!("Missing {}", path))
}
