use crate::error::Result;
use crate::messaging;
use crate::messaging::PushMessageState;
use crate::PushMessageInput;
use crate::PushMessagePayload;
use async_graphql::InputObject;
use trankeel_core::templates::CandidacyRejectedMail;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyId;
use trankeel_data::CandidacyStatus;
use trankeel_data::Discussion;
use trankeel_data::DiscussionStatus;
use trankeel_data::Message;
use trankeel_data::Person;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
pub struct RejectCandidacyInput {
    pub id: CandidacyId,
}

pub struct RejectCandidacyState {
    pub candidacy: Candidacy,
    pub discussion: Discussion,
    pub account_owner: Person,
    pub candidate: Person,
}

pub struct RejectCandidacyPayload {
    pub candidacy: Candidacy,
    pub message: Message,
    pub discussion: Discussion,
}

// # Operation

pub(crate) fn reject_candidacy(
    state: RejectCandidacyState,
    input: RejectCandidacyInput,
) -> Result<RejectCandidacyPayload> {
    input.validate()?;

    let RejectCandidacyState {
        candidacy,
        discussion,
        account_owner,
        candidate,
    } = state;

    let candidacy = Candidacy {
        id: candidacy.id,
        status: CandidacyStatus::Rejected,
        ..candidacy
    };

    let discussion = Discussion {
        id: discussion.id,
        status: DiscussionStatus::default(),
        ..discussion
    };

    let PushMessagePayload {
        message,
        discussion,
    } = messaging::push_message(
        PushMessageState {
            discussion: discussion.clone(),
        },
        PushMessageInput {
            discussion_id: discussion.id,
            sender_id: account_owner.id,
            message: CandidacyRejectedMail::try_new(&candidate)?.to_string(),
        },
    )?;

    Ok(RejectCandidacyPayload {
        candidacy,
        message,
        discussion,
    })
}
