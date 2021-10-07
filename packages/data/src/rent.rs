use crate::schema::rents;
use crate::Amount;
use crate::DateTime;
use crate::Id;
use crate::LeaseId;
use crate::PaymentNoticeId;
use crate::ReceiptId;
use crate::RentStatus;
use chrono::Duration;
use chrono::Utc;

// # Types

pub type RentId = Id;

#[derive(Clone, Insertable, Queryable)]
pub struct Rent {
    pub id: RentId,
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

impl Rent {
    pub fn delay(&self) -> Duration {
        if self.period_start.inner() > Utc::now() {
            Utc::now() - self.period_start.inner()
        } else {
            Duration::zero()
        }
    }
}

#[derive(Default, AsChangeset, Identifiable, Insertable)]
#[table_name = "rents"]
pub struct RentData {
    pub id: RentId,
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
