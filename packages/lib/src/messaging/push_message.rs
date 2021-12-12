use crate::Result;
use chrono::Utc;
use trankeel_core::dispatcher::Command;
use trankeel_data::Discussion;
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

pub struct PushMessagePayload {
    pub message: Message,
    pub discussion: Discussion,
}

pub(crate) struct PushMessage {
    pub discussion: Discussion,
}

impl PushMessage {
    pub fn new(discussion: &Discussion) -> Self {
        Self {
            discussion: discussion.clone(),
        }
    }
}

impl Command for PushMessage {
    type Input = PushMessageInput;
    type Payload = PushMessagePayload;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self { discussion } = self;

        let message = Message {
            id: MessageId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            discussion_id: input.discussion_id,
            sender_id: input.sender_id,
            content: Some(input.message),
            event_id: None,
        };

        let discussion = Discussion {
            updated_at: Some(Utc::now().into()), // Touch updated_at.
            ..discussion
        };

        Ok(Self::Payload {
            message,
            discussion,
        })
    }
}
