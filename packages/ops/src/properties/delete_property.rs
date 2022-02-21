use crate::command::Command;
use crate::error::Result;
use crate::event::Event;
use crate::event::PropertyDeleted;
use async_graphql::InputObject;
use trankeel_data::PropertyId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct DeletePropertyInput {
    pub id: PropertyId,
}

pub struct DeleteProperty;

impl Command for DeleteProperty {
    type Input = DeletePropertyInput;

    fn run(self, input: Self::Input) -> Result<Vec<Event>> {
        input.validate()?;

        Ok(vec![PropertyDeleted {
            property_id: input.id,
        }
        .into()])
    }
}
