use crate::error::Result;
use crate::Command;
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
    type Payload = PropertyId;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        Ok(input.id)
    }
}
