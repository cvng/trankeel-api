use crate::schema::events;
use crate::AccountId;
use crate::DateTime;
use crate::Eventable;
use crate::EventableId;
use crate::Id;
use crate::PersonId;

pub type EventId = Id;

pub type EventWithEventable = (Event, Eventable);

#[derive(Copy, Clone, Debug, PartialEq, Eq, DbEnum, Enum)]
#[DieselType = "Eventtype"]
pub enum EventType {
    CandidacyAccepted,
    CandidacyCreated,
    CandidacyRejected,
    LeaseCreated,
    NoticeCreated,
    NoticeSent,
    PaymentCreated,
    ReceiptCreated,
    ReceiptSent,
    StepCompleted,
}

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct Event {
    pub id: EventId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub participant_id: PersonId,
    pub eventable_id: EventableId,
    pub type_: EventType,
}
