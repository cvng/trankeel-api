use crate::error::Result;
use trankeel_core::dispatcher::Command;
use trankeel_data::LeaseId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct DeleteLeaseInput {
    pub id: LeaseId,
}

pub struct DeleteLease;

impl Command for DeleteLease {
    type Input = DeleteLeaseInput;
    type Payload = LeaseId;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        Ok(input.id)
    }
}
