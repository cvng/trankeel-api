use crate::error::Result;
use crate::messaging::push_message;
use crate::templates::CandidacyRejectedMail;
use crate::PushMessageInput;
use async_graphql::InputObject;
use trankeel_core::activity::trace;
use trankeel_core::activity::Trace;
use trankeel_core::database::Db;
use trankeel_data::AuthId;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyData;
use trankeel_data::CandidacyId;
use trankeel_data::CandidacyStatus;
use trankeel_data::DiscussionData;
use trankeel_data::DiscussionStatus;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
pub struct RejectCandidacyInput {
    pub id: CandidacyId,
}

// # Operation

pub async fn reject_candidacy(
    db: &impl Db,
    auth_id: &AuthId,
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

    let sender = db.persons().by_auth_id(auth_id)?;

    let candidate = db.persons().by_candidacy_id(&candidacy.id)?;

    push_message(
        db,
        PushMessageInput {
            discussion_id: discussion.id,
            sender_id: sender.id,
            message: CandidacyRejectedMail::try_new(&candidate)?.to_string(),
        },
    )?;

    Ok(candidacy)
}
