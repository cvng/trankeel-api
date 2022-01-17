use crate::sql_schema::rents;
use crate::Amount;
use crate::DateTime;
use crate::Id;
use crate::LeaseId;
use crate::NoticeId;
use crate::ReceiptId;
use chrono::Duration;
use chrono::Utc;

// # Types

pub type RentId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, DbEnum, Enum)]
#[DieselType = "Rentstatus"]
pub enum RentStatus {
    Open,
    Paid,
    PartiallyPaid,
}

impl Default for RentStatus {
    fn default() -> Self {
        Self::Open
    }
}

#[derive(Clone, AsChangeset, Identifiable, Insertable, Queryable, SimpleObject)]
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
    pub notice_id: Option<NoticeId>,
}

impl Rent {
    pub fn delay(&self) -> Duration {
        if self.period_start.inner() > Utc::now() {
            self.period_start.inner() - Utc::now()
        } else {
            Duration::zero()
        }
    }
}
