use crate::Result;
use chrono::Utc;
use trankeel_core::activity::trace;
use trankeel_core::activity::Trace;
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

pub struct PushMessageState {
    pub discussion: Discussion,
}

pub struct PushMessagePayload {
    pub message: Message,
    pub discussion: Discussion,
}

pub fn run_push_message(db: &impl Db, input: PushMessageInput) -> Result<Message> {
    let state = PushMessageState {
        discussion: db.discussions().by_id(&input.discussion_id)?,
    };

    let PushMessagePayload {
        message,
        discussion,
    } = push_message(input, state)?;

    trace(vec![
        Trace::MessageCreated(message.clone()),
        Trace::DiscussionUpdated(discussion),
    ])?;

    Ok(message)
}

pub fn push_message(
    input: PushMessageInput,
    state: PushMessageState,
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
        updated_at: Some(DateTime(Utc::now())), // Touch updated_at.
        ..state.discussion
    };

    Ok(PushMessagePayload {
        message,
        discussion,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_message() {
        let state = PushMessageState {
            discussion: Discussion::default(),
        };

        let input = PushMessageInput {
            discussion_id: state.discussion.id,
            sender_id: Default::default(),
            message: "test".into(),
        };

        assert!(push_message(input, state).is_ok());
    }
}
