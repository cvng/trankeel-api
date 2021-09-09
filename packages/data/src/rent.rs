use crate::common::Id;
use crate::Amount;
use crate::DateTime;
use async_graphql::Enum;
use diesel_enum_derive::DieselEnum;
use serde::Deserialize;
use serde::Serialize;

// # Types

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DieselEnum, Enum)]
pub enum RentStatus {
    Partial,
    Pending,
    Settled,
}

#[derive(Clone, Queryable)]
pub struct Rent {
    pub id: Id,
    pub period_end: DateTime,
    pub period_start: DateTime,
    pub amount: Amount,
    pub charges_amount: Option<Amount>,
    pub full_amount: Amount,
    pub status: RentStatus,
    pub lease_id: Id,
    pub receipt_id: Option<Id>,
    pub transaction_id: Option<Id>,
    pub notice_id: Option<Id>,
}
