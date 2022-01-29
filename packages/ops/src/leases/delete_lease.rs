use crate::error::Result;
use crate::event::Event;
use crate::event::LeaseDeleted;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::LeaseId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct DeleteLeaseInput {
    pub id: LeaseId,
}

pub struct DeleteLease;

impl Command for DeleteLease {
    type Input = DeleteLeaseInput;
    type Payload = Vec<Event>;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        Ok(vec![LeaseDeleted { lease_id: input.id }.into()])
    }
}
