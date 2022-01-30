use super::FurnishedLeaseDetailsInput;
use crate::error::Result;
use crate::event::Event;
use crate::event::LeaseUpdated;
use crate::files::CreateFileInput;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::Lease;
use trankeel_data::LeaseId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct UpdateFurnishedLeaseInput {
    pub details: Option<FurnishedLeaseDetailsInput>,
    pub file: Option<CreateFileInput>, // TODO
    pub id: LeaseId,
}

pub struct UpdateFurnishedLease {
    lease: Lease,
}

impl UpdateFurnishedLease {
    pub fn new(lease: &Lease) -> Self {
        Self {
            lease: lease.clone(),
        }
    }
}

impl Command for UpdateFurnishedLease {
    type Input = UpdateFurnishedLeaseInput;

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
