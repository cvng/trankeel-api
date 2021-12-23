use crate::error::Result;
use crate::Command;
use trankeel_core::dispatcher::Event;
use trankeel_core::handlers::DiscussionDeleted;
use trankeel_data::DiscussionId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct DeleteDiscussionInput {
    pub id: DiscussionId,
}

pub struct DeleteDiscussionCommand;

impl Command for DeleteDiscussionCommand {
    type Input = DeleteDiscussionInput;
    type Payload = Vec<Event>;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        Ok(vec![DiscussionDeleted::with(input.id)])
    }
}
