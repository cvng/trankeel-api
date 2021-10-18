use crate::Command;
use crate::Result;
use chrono::Utc;
use trankeel_core::database::Db;
use trankeel_data::DateTime;
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

pub struct State {
    pub discussion: Discussion,
}

pub struct Output {
    pub message: Message,
    pub discussion: Discussion,
}

pub(crate) struct PushMessageCommand;

impl Command for PushMessageCommand {
    type Input = PushMessageInput;
    type State = State;
    type Output = Output;

    fn run(state: Self::State, input: Self::Input) -> Result<Self::Output> {
        input.validate()?;

        let mut discussion = state.discussion;

        let message = Message {
            id: MessageId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            discussion_id: input.discussion_id,
            sender_id: input.sender_id,
            content: Some(input.message),
            event_id: None,
        };

        discussion.updated_at = Some(DateTime(Utc::now())); // Touch updated_at.

        Ok(Output {
            message,
            discussion,
        })
    }
}

pub fn push_message(db: &impl Db, input: PushMessageInput) -> Result<Message> {
    input.validate()?;

    let message = db.messages().create(Message {
        id: MessageId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        discussion_id: input.discussion_id,
        sender_id: input.sender_id,
        content: Some(input.message),
        event_id: None,
    })?;

    db.discussions().touch(input.discussion_id)?; // Touch updated_at.

    Ok(message)
}
