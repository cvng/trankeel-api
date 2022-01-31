use crate::id;
use crate::sql_schema::messages;
use crate::DateTime;
use crate::DiscussionId;
use crate::EventId;
use crate::PersonId;
use fake::Fake;

id!(MessageId);

pub type MessageContent = String;

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct Message {
    pub id: MessageId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub discussion_id: DiscussionId,
    pub sender_id: PersonId,
    pub content: Option<MessageContent>,
    pub event_id: Option<EventId>,
}
