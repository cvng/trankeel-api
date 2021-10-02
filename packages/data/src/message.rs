use crate::schema::messages;
use crate::DateTime;
use crate::DiscussionId;
use crate::Id;
use crate::PersonId;

pub type MessageId = Id;

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct Message {
    pub id: MessageId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub discussion_id: DiscussionId,
    pub sender_id: PersonId,
    pub content: String,
}
