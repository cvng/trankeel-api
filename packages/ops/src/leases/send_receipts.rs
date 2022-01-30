use crate::error::Result;
use crate::event::Event;
use crate::event::ReceiptSent;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::RentId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct SendReceiptsInput {
    pub rent_ids: Vec<RentId>,
}

pub struct SendReceipts;

impl Command for SendReceipts {
    type Input = SendReceiptsInput;

    fn run(self, input: Self::Input) -> Result<Vec<Event>> {
        input.validate()?;

        Ok(input
            .rent_ids
            .into_iter()
            .map(|rent_id| ReceiptSent { rent_id }.into())
            .collect())
    }
}
