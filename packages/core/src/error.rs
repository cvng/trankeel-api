pub use eyre::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub fn no(path: &str) -> Error {
    Error::msg(format!("Missing {}", path))
}

#[derive(Debug, thiserror::Error)]
pub enum InternalError {
    #[error("mailer error")]
    MailerError,
    #[error("pdfmaker error")]
    PdfmakerError(#[from] reqwest::Error),
}
