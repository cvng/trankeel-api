use super::reject_candidacy;
use super::RejectCandidacyInput;
use crate::error::Result;
use async_graphql::InputObject;
use piteo_core::database::Db;
use piteo_data::AuthId;
use piteo_data::Candidacy;
use piteo_data::CandidacyData;
use piteo_data::CandidacyId;
use piteo_data::CandidacyStatus;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
pub struct AcceptCandidacyInput {
    pub id: CandidacyId,
}

// # Operation

pub fn accept_candidacy(
    db: &impl Db,
    auth_id: &AuthId,
    input: AcceptCandidacyInput,
) -> Result<Candidacy> {
    input.validate()?;

    let advertisement = db.advertisements().by_candidacy_id(&input.id)?;

    // Reject other candidacies.
    let other_candidacies = db.candidacies().by_advertisement_id(&advertisement.id)?;

    for candidacy in other_candidacies {
        reject_candidacy(db, auth_id, RejectCandidacyInput { id: candidacy.id })?;
    }

    // Accept given candidacy.
    let candidacy = db.candidacies().update(CandidacyData {
        id: input.id,
        status: Some(CandidacyStatus::Accepted),
        ..Default::default()
    })?;

    Ok(candidacy)
}
