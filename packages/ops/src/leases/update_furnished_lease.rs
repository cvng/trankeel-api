use super::FurnishedLeaseDetailsInput;
use crate::error::Result;
use crate::files::CreateFileInput;
use async_graphql::InputObject;
use trankeel_core::dispatcher::Command;
use trankeel_data::Lease;
use trankeel_data::LeaseId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct UpdateFurnishedLeaseInput {
    pub details: Option<FurnishedLeaseDetailsInput>,
    pub file: Option<CreateFileInput>, // TODO
    pub id: LeaseId,
}

pub struct UpdateFurnishedLeasePayload {
    pub lease: Lease,
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
    type Payload = UpdateFurnishedLeasePayload;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self { lease } = self;

        let lease = Lease {
            id: input.id,
            details: input.details.map(Into::into),
            ..lease
        };

        Ok(Self::Payload { lease })
    }
}
