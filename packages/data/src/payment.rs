use crate::schema::payments;
use crate::Amount;
use crate::DateTime;
use crate::Id;
use crate::RentId;

// # Types

pub type PaymentId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DbEnum, Enum)]
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

#[derive(Clone, Serialize, Deserialize, Insertable, Queryable, SimpleObject)]
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
