use crate::common::Id;
use crate::schema::rents;
use crate::Amount;
use crate::DateTime;
use crate::LeaseId;
use crate::PaymentNoticeId;
use crate::ReceiptId;

// # Types

pub type RentId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DieselEnum, Enum)]
pub enum RentStatus {
    Partial,
    Pending,
    Settled,
}

#[derive(Clone, Insertable, Queryable)]
pub struct Rent {
    pub id: Id,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub period_end: DateTime,
    pub period_start: DateTime,
    pub amount: Amount,
    pub charges_amount: Option<Amount>,
    pub full_amount: Amount,
    pub status: RentStatus,
    pub lease_id: LeaseId,
    pub receipt_id: Option<ReceiptId>,
    pub notice_id: Option<PaymentNoticeId>,
}

#[derive(Default, Deserialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "rents"]
pub struct RentData {
    pub id: Id,
    pub period_end: Option<DateTime>,
    pub period_start: Option<DateTime>,
    pub amount: Option<Amount>,
    pub charges_amount: Option<Amount>,
    pub full_amount: Option<Amount>,
    pub status: Option<RentStatus>,
    pub lease_id: Option<LeaseId>,
    pub receipt_id: Option<ReceiptId>,
    pub notice_id: Option<PaymentNoticeId>,
}
