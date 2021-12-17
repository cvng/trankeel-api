use crate::error::Result;
use async_graphql::InputObject;
use trankeel_core::dispatcher::Command;
use trankeel_data::RentId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct SendReceiptsInput {
    pub rent_ids: Vec<RentId>,
}

pub struct SendReceipts;

impl Command for SendReceipts {
    type Input = SendReceiptsInput;
    type Payload = Vec<RentId>;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        Ok(input.rent_ids)
    }
}
