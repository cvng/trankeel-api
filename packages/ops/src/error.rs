pub use eyre::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum DomainError {
    #[error("invite reason `{0}` is not implemented")]
    UnimplementedInviteReason(trankeel_data::InviteReason),
    #[error("candidacy `{0}` is already rejected")]
    CandidacyAlreadyRejected(trankeel_data::CandidacyId),
}
