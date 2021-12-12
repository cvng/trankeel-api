use crate::Result;
use trankeel_core::dispatcher::Command;
use trankeel_data::DiscussionId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct DeleteDiscussionInput {
    pub id: DiscussionId,
}

pub(crate) struct DeleteDiscussion;

impl Command for DeleteDiscussion {
    type Input = DeleteDiscussionInput;
    type Payload = DiscussionId;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        Ok(input.id)
    }
}
