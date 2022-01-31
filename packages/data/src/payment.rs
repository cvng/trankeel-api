use crate::id;
use crate::sql_schema::payments;
use crate::Amount;
use crate::DateTime;
use crate::RentId;
use async_graphql::Enum;
use async_graphql::SimpleObject;
use diesel_derive_enum::DbEnum;
use fake::Fake;

// # Types

id!(PaymentId);

#[derive(Copy, Clone, Debug, PartialEq, Eq, DbEnum, Enum)]
#[DieselType = "Transactiontype"]
pub enum TransactionType {
    InsuranceHab,
    InsurancePno,
    Invoice,
    LoanInterest,
    LoanPayment,
    Other,
    Rent,
}

#[derive(Clone, Insertable, Queryable, SimpleObject)]
pub struct Payment {
    pub id: PaymentId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub rent_id: RentId,
    pub amount: Amount,
    pub date: DateTime,
    pub type_: TransactionType,
    pub label: Option<String>,
}
