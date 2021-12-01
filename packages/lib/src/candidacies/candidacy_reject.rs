use crate::error::Result;
use crate::messaging;
use crate::messaging::PushMessageState;
use crate::PushMessageInput;
use crate::PushMessagePayload;
use async_graphql::InputObject;
use trankeel_core::context::Context;
use trankeel_core::database::Db;
use trankeel_core::dispatcher::dispatch;
use trankeel_core::dispatcher::Command;
use trankeel_core::dispatcher::Event;
use trankeel_core::templates::CandidacyRejectedMail;
use trankeel_data::AuthId;
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

pub(crate) struct RejectCandidacy<'a> {
    context: &'a Context,
    auth_id: &'a AuthId,
}

impl<'a> RejectCandidacy<'a> {
    pub fn new(context: &'a Context, auth_id: &'a AuthId) -> Self {
        Self { context, auth_id }
    }
}

#[async_trait]
impl<'a> Command for RejectCandidacy<'a> {
    type Input = RejectCandidacyInput;
    type Payload = RejectCandidacyPayload;

    async fn run(&self, input: Self::Input) -> Result<Self::Payload> {
        let db = self.context.db();

        let candidacy = db.candidacies().by_id(&input.id)?;
        let discussion = db.discussions().by_candidacy_id(&candidacy.id)?;
        let account_owner = db.persons().by_auth_id(self.auth_id)?;
        let candidate = db.persons().by_candidacy_id(&candidacy.id)?;

        let state = RejectCandidacyState {
            candidacy,
            discussion,
            account_owner,
            candidate,
        };

        let payload = reject_candidacy(state, input)?;

        db.transaction(|| {
            db.candidacies().update(&payload.candidacy)?;
            db.discussions().update(&payload.discussion)?;
            db.messages().create(&payload.message)?;
            dispatch(vec![Event::CandidacyRejected(payload.candidacy.clone())])?;
            Ok(())
        })?;

        Ok(payload)
    }
}

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
