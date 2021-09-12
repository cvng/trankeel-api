use crate::common::Id;
use crate::schema::rent;
use crate::Amount;
use crate::Attachable;
use crate::DateTime;
use crate::FileId;
use crate::LeaseId;
use crate::PaymentNoticeId;
use crate::ReceiptId;
use crate::TransactionId;
use async_graphql::Enum;
use diesel_enum_derive::DieselEnum;
use serde::Deserialize;
use serde::Serialize;

// # Types

pub type RentId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DieselEnum, Enum)]
pub enum RentStatus {
    Partial,
    Pending,
    Settled,
}

#[derive(Clone, Insertable, Queryable)]
#[table_name = "rent"]
pub struct Rent {
    pub id: Id,
    pub period_end: DateTime,
    pub period_start: DateTime,
    pub amount: Amount,
    pub charges_amount: Option<Amount>,
    pub full_amount: Amount,
    pub status: RentStatus,
    pub lease_id: LeaseId,
    pub receipt_id: Option<ReceiptId>,
    pub transaction_id: Option<TransactionId>,
    pub notice_id: Option<PaymentNoticeId>,
}

#[derive(Default, Deserialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "rent"]
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
    pub transaction_id: Option<TransactionId>,
    pub notice_id: Option<PaymentNoticeId>,
}

// # Impls

impl Attachable for Rent {
    fn to_filename(&self, file_id: &FileId) -> String {
        let id = file_id.to_string();
        let id = id.split('-').next().unwrap_or("id");
        let date = self.period_start.inner().to_rfc3339();
        let date = date.split('T').next().unwrap_or("date");
        format!("{}-quittance-{}.pdf", &date, &id) // Ex: "07-21-quittance-ab60265a.pdf"
    }
}
