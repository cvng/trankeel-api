// #![allow(dead_code)]

// mod adapter;
// mod configuration;
// mod document;

pub use super::adapter::Adapter;
pub use super::configuration::Configuration;
pub use super::document::Document;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub errors: Vec<String>,
    pub status: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status, self.errors.join(", "))
    }
}

impl std::error::Error for Error {}

#[derive(Clone)]
pub struct Pdfmonkey {
    pub(crate) config: Configuration,
}
