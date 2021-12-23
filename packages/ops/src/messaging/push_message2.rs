use super::PushMessageInput;
use crate::error::Result;
use crate::Command;
use trankeel_core::dispatcher::Event;
use trankeel_core::handlers::MessagePushed;
use trankeel_data::Message;
use trankeel_data::MessageId;
use validator::Validate;

pub struct PushMessageCommand {
    pub message_id: MessageId,
}

impl PushMessageCommand {
    pub fn new(message_id: &MessageId) -> Self {
        Self {
            message_id: *message_id,
        }
    }
}

impl Command for PushMessageCommand {
    type Input = PushMessageInput;
    type Payload = Vec<Event>;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
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

        Ok(vec![MessagePushed::with(&message)])
    }
}
