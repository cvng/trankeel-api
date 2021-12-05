use crate::error::Result;
use crate::messaging;
use crate::messaging::PushMessageState;
use crate::PushMessageInput;
use crate::PushMessagePayload;
use async_graphql::InputObject;
use trankeel_core::dispatcher::Command;
use trankeel_core::dispatcher::Event;
use trankeel_core::handlers::CandidacyRejected;
use trankeel_core::templates::CandidacyRejectedMail;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyId;
use trankeel_data::CandidacyStatus;
use trankeel_data::Discussion;
use trankeel_data::DiscussionStatus;
use trankeel_data::Person;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct RejectCandidacyInput {
    pub id: CandidacyId,
}

pub(crate) struct RejectCandidacy<'a> {
    candidacy: &'a Candidacy,
    candidate: &'a Person,
    account_owner: &'a Person,
    discussion: &'a Discussion,
}

impl<'a> RejectCandidacy<'a> {
    pub fn new(
        candidacy: &'a Candidacy,
        candidate: &'a Person,
        account_owner: &'a Person,
        discussion: &'a Discussion,
    ) -> Self {
        Self {
            candidacy,
            candidate,
            account_owner,
            discussion,
        }
    }

    pub fn reject_candidacy(&self, input: RejectCandidacyInput) -> Result<CandidacyRejected> {
        input.validate()?;

        let candidacy = Candidacy {
            status: CandidacyStatus::Rejected,
            ..self.candidacy.clone()
        };

        let discussion = Discussion {
            status: DiscussionStatus::default(),
            ..self.discussion.clone()
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
                sender_id: self.account_owner.id,
                message: CandidacyRejectedMail::try_new(self.candidate)?.to_string(),
            },
        )?;

        Ok(CandidacyRejected {
            candidacy,
            message,
            discussion,
        })
    }
}

#[async_trait]
impl<'a> Command for RejectCandidacy<'a> {
    type Input = RejectCandidacyInput;
    type Payload = Vec<Event>;

    async fn run(&self, input: Self::Input) -> Result<Self::Payload> {
        let payload = self.reject_candidacy(input)?;

        Ok(vec![Event::CandidacyRejected(payload)])
    }
}
