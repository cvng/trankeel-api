use super::LeaseDetailsInput;
use crate::error::Result;
use crate::event::Event;
use crate::event::LeaseUpdated;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::Lease;
use trankeel_data::LeaseId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct UpdateLeaseInput {
    pub details: Option<LeaseDetailsInput>,
    pub id: LeaseId,
}

pub struct UpdateLease {
    lease: Lease,
}

impl UpdateLease {
    pub fn new(lease: &Lease) -> Self {
        Self {
            lease: lease.clone(),
        }
    }
}

impl Command for UpdateLease {
    type Input = UpdateLeaseInput;

    fn run(self, input: Self::Input) -> Result<Vec<Event>> {
        input.validate()?;

        let Self { lease } = self;

        let lease = Lease {
            id: input.id,
            details: input.details.map(Into::into),
            ..lease
        };

        Ok(vec![LeaseUpdated { lease }.into()])
    }
}
