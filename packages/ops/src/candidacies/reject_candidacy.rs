use crate::error::Result;
use crate::messaging::PushMessage;
use crate::messaging::PushMessageInput;
use crate::messaging::PushMessagePayload;
use async_graphql::InputObject;
use trankeel_core::dispatcher::Command;
use trankeel_core::templates::CandidacyRejectedMail;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyId;
use trankeel_data::CandidacyStatus;
use trankeel_data::Discussion;
use trankeel_data::DiscussionStatus;
use trankeel_data::Message;
use trankeel_data::Person;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct RejectCandidacyInput {
    pub id: CandidacyId,
}

pub struct RejectCandidacyPayload {
    pub candidacy: Candidacy,
    pub message: Message,
    pub discussion: Discussion,
}

pub struct RejectCandidacy {
    candidacy: Candidacy,
    candidate: Person,
    account_owner: Person,
    discussion: Discussion,
}

impl RejectCandidacy {
    pub fn new(
        candidacy: &Candidacy,
        candidate: &Person,
        account_owner: &Person,
        discussion: &Discussion,
    ) -> Self {
        Self {
            candidacy: candidacy.clone(),
            candidate: candidate.clone(),
            account_owner: account_owner.clone(),
            discussion: discussion.clone(),
        }
    }
}

impl Command for RejectCandidacy {
    type Input = RejectCandidacyInput;
    type Payload = RejectCandidacyPayload;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self {
            candidacy,
            candidate,
            account_owner,
            discussion,
        } = self;

        let candidacy = Candidacy {
            status: CandidacyStatus::Rejected,
            ..candidacy
        };

        let discussion = Discussion {
            status: DiscussionStatus::default(),
            ..discussion
        };

        let PushMessagePayload {
            message,
            discussion,
        } = PushMessage::new(&discussion).run(PushMessageInput {
            discussion_id: discussion.id,
            sender_id: account_owner.id,
            message: CandidacyRejectedMail::try_new(&candidate)?.to_string(),
        })?;

        Ok(Self::Payload {
            candidacy,
            message,
            discussion,
        })
    }
}
