use crate::schema::payments;
use crate::Amount;
use crate::DateTime;
use crate::Id;
use crate::RentId;
use async_graphql::Enum;
use diesel_enum_derive::DieselEnum;

// # Types

pub type PaymentId = Id;

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

#[derive(Clone, Insertable, Queryable)]
pub struct Payment {
    pub id: PaymentId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub rent_id: RentId,
    pub amount: Amount,
    pub date: DateTime,
    pub type_: TransactionType,
    pub label: String,
}
