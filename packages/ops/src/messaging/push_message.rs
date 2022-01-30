use crate::error::Result;
use crate::event::Event;
use crate::event::MessagePushed;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::DiscussionId;
use trankeel_data::Message;
use trankeel_data::MessageId;
use trankeel_data::PersonId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct PushMessageInput {
    pub discussion_id: DiscussionId,
    pub sender_id: PersonId,
    pub message: String,
}

pub struct PushMessage {
    pub message_id: MessageId,
}

impl PushMessage {
    pub fn new(message_id: MessageId) -> Self {
        Self { message_id }
    }
}

impl Command for PushMessage {
    type Input = PushMessageInput;

    fn run(self, input: Self::Input) -> Result<Vec<Event>> {
        input.validate()?;

        let Self { message_id } = self;

        let message = Message {
            id: message_id,
            created_at: Default::default(),
            updated_at: Default::default(),
            discussion_id: input.discussion_id,
            sender_id: input.sender_id,
            content: Some(input.message),
            event_id: None,
        };

        Ok(vec![MessagePushed { message }.into()])
    }
}
