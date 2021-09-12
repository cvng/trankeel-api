use crate::AccountId;
use crate::Amount;
use crate::DateTime;
use crate::Id;
use crate::LeaseId;
use async_graphql::Enum;
use diesel_enum_derive::DieselEnum;

// # Types

pub type TransactionId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, DieselEnum, Enum)]
pub enum TransactionType {
    InsuranceHab,
    InsurancePno,
    Invoice,
    LoanInterest,
    LoanPayment,
    Other,
    Rent,
}

#[derive(Clone, Queryable)]
pub struct Transaction {
    pub account_id: AccountId,
    pub amount: Amount,
    pub lease_id: LeaseId,
    pub date: DateTime,
    pub label: String,
    pub type_: TransactionType,
    pub id: TransactionId,
}
