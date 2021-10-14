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
    CandidacyCreated,
    #[graphql(name = "PAYMENT_NOTICE_CREATED")]
    NoticeCreated,
    #[graphql(name = "PAYMENT_NOTICE_SENT")]
    NoticeSent,
    #[graphql(name = "RENT_RECEIPT_CREATED")]
    ReceiptCreated,
    #[graphql(name = "RENT_RECEIPT_SENT")]
    ReceiptSent,
    #[graphql(name = "TRANSACTION_CREATED")]
    PaymentCreated,
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
