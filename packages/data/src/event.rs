use crate::AccountId;
use crate::DateTime;
use crate::Id;
use async_graphql::Enum;
use diesel_enum_derive::DieselEnum;

// # Types

pub type EventId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, DieselEnum, Enum)]
pub enum EventType {
    RentReceiptCreated,
    RentReceiptSent,
    TransactionCreated,
}

pub struct Event {
    pub id: EventId,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub account_id: AccountId,
    pub eventable_id: Id,
    pub eventable_type: String,
    pub r#type: EventType,
}
