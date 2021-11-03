use crate::schema::events;
use crate::DateTime;
use crate::Id;
use serde_json::Value;

pub type EventId = Id;

#[derive(Clone, Default, Debug, Serialize, Deserialize, DieselNewType)]
pub struct EventPayload(Value);

impl From<Value> for EventPayload {
    fn from(item: Value) -> Self {
        Self(item)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, DbEnum, Enum)]
#[DieselType = "Eventtype"]
pub enum EventType {
    CandidacyCreated,
    CandidacyAccepted,
    CandidacyRejected,
    DiscussionCreated,
    DiscussionUpdated,
    LeaseCreated,
    MessageCreated,
    NoticeCreated,
    NoticeSent,
    PaymentCreated,
    ReceiptCreated,
    ReceiptSent,
    StepCompleted,
    TenantCreated,
    WarrantCreated,
}

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct Event {
    pub id: EventId,
    pub created_at: Option<DateTime>,
    pub type_: EventType,
    pub payload: EventPayload,
}
