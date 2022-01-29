use crate::error::Result;
use crate::event::DiscussionDeleted;
use crate::event::Event;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::DiscussionId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct DeleteDiscussionInput {
    pub id: DiscussionId,
}

pub struct DeleteDiscussion;

impl Command for DeleteDiscussion {
    type Input = DeleteDiscussionInput;
    type Payload = Vec<Event>;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        Ok(vec![DiscussionDeleted {
            discussion_id: input.id,
        }
        .into()])
    }
}
