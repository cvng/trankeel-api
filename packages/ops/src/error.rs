pub use eyre::Error;

pub type Result<T> = std::result::Result<T, Error>;
