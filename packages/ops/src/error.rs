pub use eyre::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("the invite reason `{0}` is not implemented")]
    InviteReasonUnimplemented(trankeel_data::InviteReason),
}
