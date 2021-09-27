use crate::schema::events;
use crate::AccountId;
use crate::DateTime;
use crate::Id;
use crate::Payment;
use crate::Rent;

// # Types

pub type EventId = Id;

pub type EventableId = Id;

pub type EventWithEventable = (Event, Eventable);

#[derive(Copy, Clone, Debug, PartialEq, Eq, DieselEnum, Enum)]
pub enum EventType {
    PaymentNoticeCreated,
    PaymentNoticeSent,
    RentReceiptCreated,
    RentReceiptSent,
    #[graphql(name = "TRANSACTION_CREATED")]
    PaymentCreated,
}

#[derive(Clone)]
pub enum Eventable {
    Rent(Rent),
    Payment(Payment),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, DieselEnum, Enum)]
pub enum EventableType {
    Rent,
    Payment,
}

impl From<EventType> for EventableType {
    fn from(item: EventType) -> Self {
        match item {
            EventType::PaymentNoticeCreated => Self::Rent,
            EventType::PaymentNoticeSent => Self::Rent,
            EventType::RentReceiptCreated => Self::Rent,
            EventType::RentReceiptSent => Self::Rent,
            EventType::PaymentCreated => Self::Payment,
        }
    }
}

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct Event {
    pub id: EventId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub eventable_id: EventableId,
    pub eventable_type: EventableType,
    pub type_: EventType,
}
