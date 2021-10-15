use crate::error::Result;
use async_graphql::InputObject;
use piteo_core::activity::trace;
use piteo_core::activity::Trace;
use piteo_core::database::Db;
use piteo_data::AuthId;
use piteo_data::Candidacy;
use piteo_data::CandidacyData;
use piteo_data::CandidacyId;
use piteo_data::CandidacyStatus;
use piteo_data::DiscussionData;
use piteo_data::DiscussionStatus;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
pub struct RejectCandidacyInput {
    pub id: CandidacyId,
}

// # Operation

pub fn reject_candidacy(
    db: &impl Db,
    _auth_id: &AuthId,
    input: RejectCandidacyInput,
) -> Result<Candidacy> {
    input.validate()?;

    let candidacy = db.candidacies().by_id(&input.id)?;

    db.candidacies().update(CandidacyData {
        id: candidacy.id,
        status: Some(CandidacyStatus::Rejected),
        ..Default::default()
    })?;

    trace(db, Trace::CandidacyRejected(candidacy.clone())).ok();

    let discussion = db.discussions().by_candidacy_id(&candidacy.id)?;

    db.discussions().update(DiscussionData {
        id: discussion.id,
        status: Some(DiscussionStatus::default()),
        ..Default::default()
    })?;

    Ok(candidacy)
}
