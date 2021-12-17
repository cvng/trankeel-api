use crate::error::Result;
use crate::messaging::PushMessage;
use crate::messaging::PushMessageInput;
use crate::messaging::PushMessagePayload;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyId;
use trankeel_data::CandidacyStatus;
use trankeel_data::Discussion;
use trankeel_data::DiscussionStatus;
use trankeel_data::Message;
use trankeel_data::MessageContent;
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
    account_owner: Person,
    discussion: Discussion,
    candidacy_rejected_message: MessageContent,
}

impl RejectCandidacy {
    pub fn new(
        candidacy: &Candidacy,
        account_owner: &Person,
        discussion: &Discussion,
        candidacy_rejected_message: &str,
    ) -> Self {
        Self {
            candidacy: candidacy.clone(),
            account_owner: account_owner.clone(),
            discussion: discussion.clone(),
            candidacy_rejected_message: candidacy_rejected_message.to_string(),
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
            account_owner,
            discussion,
            candidacy_rejected_message,
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
            message: candidacy_rejected_message,
        })?;

        Ok(Self::Payload {
            candidacy,
            message,
            discussion,
        })
    }
}
