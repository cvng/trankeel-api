use crate::client::Actor;
use crate::client::Context;
use crate::commands;
use crate::error::Result;
use crate::templates::CandidacyRejectedMail;
use crate::PushMessageInput;
use async_graphql::InputObject;
use trankeel_core::activity::trace;
use trankeel_core::activity::Trace;
use trankeel_core::database::Db;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyId;
use trankeel_data::CandidacyStatus;
use trankeel_data::Discussion;
use trankeel_data::DiscussionStatus;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
pub struct RejectCandidacyInput {
    pub id: CandidacyId,
}

// # Operation

pub(crate) async fn reject_candidacy(
    ctx: &Context,
    actor: &Actor,
    input: RejectCandidacyInput,
) -> Result<Candidacy> {
    let db = ctx.db();
    let auth_id = actor.check()?;

    input.validate()?;

    let candidacy = db.candidacies().by_id(&input.id)?;

    db.candidacies().update(&Candidacy {
        id: candidacy.id,
        status: CandidacyStatus::Rejected,
        ..candidacy.clone()
    })?;

    trace(db, Trace::CandidacyRejected(candidacy.clone()))?;

    let discussion = db.discussions().by_candidacy_id(&candidacy.id)?;

    db.discussions().update(&Discussion {
        id: discussion.id,
        status: DiscussionStatus::default(),
        ..discussion
    })?;

    let sender = db.persons().by_auth_id(auth_id)?;

    let candidate = db.persons().by_candidacy_id(&candidacy.id)?;

    commands::push_message(
        ctx,
        actor,
        PushMessageInput {
            discussion_id: discussion.id,
            sender_id: sender.id,
            message: CandidacyRejectedMail::try_new(&candidate)?.to_string(),
        },
    )?;

    Ok(candidacy)
}
