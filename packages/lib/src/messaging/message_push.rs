use crate::Command;
use crate::Result;
use chrono::Utc;
use trankeel_core::context::Context;
use trankeel_core::database::Db;
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

pub struct PushMessageState {
    pub discussion: Discussion,
}

pub struct PushMessagePayload {
    pub message: Message,
    pub discussion: Discussion,
}

pub(crate) struct PushMessage<'a> {
    context: &'a Context,
}

impl<'a> PushMessage<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }
}

#[async_trait]
impl<'a> Command for PushMessage<'a> {
    type Input = PushMessageInput;
    type Payload = PushMessagePayload;

    async fn run(&self, input: Self::Input) -> Result<Self::Payload> {
        let db = self.context.db();

        let state = PushMessageState {
            discussion: db.discussions().by_id(&input.discussion_id)?,
        };

        let payload = push_message(state, input)?;

        db.transaction(|| {
            db.messages().create(&payload.message)?;
            db.discussions().update(&payload.discussion)?;
            Ok(())
        })?;

        Ok(payload)
    }
}

pub fn push_message(
    state: PushMessageState,
    input: PushMessageInput,
) -> Result<PushMessagePayload> {
    input.validate()?;

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
        ..state.discussion
    };

    Ok(PushMessagePayload {
        message,
        discussion,
    })
}
