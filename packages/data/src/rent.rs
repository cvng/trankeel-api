use crate::id;
use crate::sql_schema::rents;
use crate::Amount;
use crate::DateTime;
use crate::LeaseId;
use crate::NoticeId;
use crate::ReceiptId;
use async_graphql::Enum;
use async_graphql::SimpleObject;
use chrono::Duration;
use chrono::Utc;
use diesel_derive_enum::DbEnum;
use fake::Fake;
use serde::Serialize;

// # Types

id!(RentId);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, DbEnum, Enum)]
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

#[derive(Clone, Serialize, AsChangeset, Identifiable, Insertable, Queryable, SimpleObject)]
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
