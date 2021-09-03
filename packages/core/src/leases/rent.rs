use crate::Amount;
use crate::DateTime;
use crate::Id;
use piteo_data::RentStatus;

// # Models

#[derive(Queryable)]
pub struct Rent {
    pub id: Id,
    pub period_end: DateTime,
    pub period_start: DateTime,
    pub amount: Amount,
    pub charges_amount: Option<Amount>,
    pub full_amount: Amount,
    pub status: RentStatus,
    pub account_id: Id,
    pub lease_id: Id,
    pub receipt_id: Option<Id>,
    pub transaction_id: Option<Id>,
    pub notice_id: Option<Id>,
}
