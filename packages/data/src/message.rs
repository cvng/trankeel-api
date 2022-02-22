use crate::id;
use crate::sql_schema::messages;
use crate::DateTime;
use crate::DiscussionId;
use crate::EventType;
use crate::EventableId;
use crate::PersonId;
use fake::Fake;
use serde::Serialize;

id!(MessageId);

pub type MessageContent = String;

#[derive(Clone, Debug, Serialize, Insertable, Queryable)]
pub struct Message {
    pub id: MessageId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub discussion_id: DiscussionId,
    pub sender_id: PersonId,
    pub content: Option<MessageContent>,
    pub type_: Option<EventType>,
    pub eventable_id: Option<EventableId>,
}
