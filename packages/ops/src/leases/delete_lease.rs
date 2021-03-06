use crate::command::Command;
use crate::error::Result;
use crate::event::Event;
use crate::event::LeaseDeleted;
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

    fn run(self, input: Self::Input) -> Result<Vec<Event>> {
        input.validate()?;

        Ok(vec![LeaseDeleted { lease_id: input.id }.into()])
    }
}
