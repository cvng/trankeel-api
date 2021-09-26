use async_graphql::InputObject;
use piteo_core::database::Db;
use piteo_core::error::Error;
use piteo_data::AuthId;
use piteo_data::Candidacy;
use piteo_data::CandidacyData;
use piteo_data::CandidacyId;
use piteo_data::CandidacyStatus;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
pub struct AcceptCandidacyInput {
    id: CandidacyId,
}

// # Operation

pub fn accept_candidacy(
    db: &impl Db,
    _auth_id: &AuthId,
    input: AcceptCandidacyInput,
) -> Result<Candidacy, Error> {
    input.validate()?;

    let advertisement = db.advertisements().by_candidacy_id(&input.id)?;

    // Reject other candidacies.
    db.candidacies().update_by_advertisement_id(
        &advertisement.id,
        CandidacyData {
            status: Some(CandidacyStatus::Rejected),
            ..Default::default()
        },
    )?;

    // Accept given candidacy.
    let candidacy = db.candidacies().update(CandidacyData {
        id: input.id,
        status: Some(CandidacyStatus::Accepted),
        ..Default::default()
    })?;

    Ok(candidacy)
}
