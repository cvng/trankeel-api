use crate::Result;
use trankeel_core::database::Db;
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
