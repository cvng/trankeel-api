use crate::id;
use crate::sql_schema::events;
use crate::AccountId;
use crate::DateTime;
use crate::Eventable;
use crate::EventableId;
use crate::PersonId;
use async_graphql::Enum;
use diesel_derive_enum::DbEnum;
use fake::Fake;
use serde::Serialize;

id!(EventId);

pub type EventWithEventable = (Event, Eventable);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, DbEnum, Enum)]
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

#[derive(Clone, Debug, Serialize, Insertable, Queryable)]
pub struct Event {
    pub id: EventId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub participant_id: PersonId,
    pub eventable_id: EventableId,
    pub type_: EventType,
    // TODO: pub payload: Value,
}
