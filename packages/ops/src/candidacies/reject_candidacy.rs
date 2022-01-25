use crate::error::DomainError;
use crate::error::Result;
use crate::event::CandidacyRejected;
use crate::event::Event;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyId;
use trankeel_data::CandidacyStatus;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct RejectCandidacyInput {
    pub id: CandidacyId,
}

pub struct RejectCandidacy {
    candidacy: Candidacy,
}

impl RejectCandidacy {
    pub fn new(candidacy: &Candidacy) -> Self {
        Self {
            candidacy: candidacy.clone(),
        }
    }
}

impl Command for RejectCandidacy {
    type Input = RejectCandidacyInput;
    type Payload = Vec<Event>;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self { candidacy } = self;

        if candidacy.status == CandidacyStatus::Rejected {
            return Err(DomainError::CandidacyAlreadyRejected(candidacy.id).into());
        }

        Ok(vec![CandidacyRejected {
            candidacy_id: candidacy.id,
        }
        .into()])
    }
}
