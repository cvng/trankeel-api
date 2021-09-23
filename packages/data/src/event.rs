use crate::schema::events;
use crate::AccountId;
use crate::DateTime;
use crate::Id;
use crate::Payment;
use crate::Rent;
use async_graphql::Enum;
use diesel_enum_derive::DieselEnum;

// # Types

pub type EventId = Id;

pub type EventableId = Id;

#[derive(Clone)]
pub enum DetailedEvent {
    Rent(Event, Rent),
    Payment(Event, Payment),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, DieselEnum, Enum)]
pub enum EventType {
    RentReceiptCreated,
    RentReceiptSent,
    #[graphql(name = "TRANSACTION_CREATED")]
    PaymentCreated,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, DieselEnum, Enum)]
pub enum EventableModel {
    Rent,
    Payment,
}

impl From<EventType> for EventableModel {
    fn from(item: EventType) -> Self {
        match item {
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
    pub eventable_model: EventableModel,
    pub type_: EventType,
}
