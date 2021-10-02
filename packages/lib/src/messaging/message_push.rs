use crate::Result;
use piteo_core::database::Db;
use piteo_data::DiscussionId;
use piteo_data::Message;
use piteo_data::MessageId;
use piteo_data::PersonId;
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
        id: MessageId::new_v4(),
        created_at: Default::default(),
        updated_at: Default::default(),
        discussion_id: input.discussion_id,
        sender_id: input.sender_id,
        content: input.message,
    })?;

    db.discussions().touch(input.discussion_id)?; // Touch updated_at.

    Ok(message)
}
