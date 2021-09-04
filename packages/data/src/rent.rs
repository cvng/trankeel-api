use crate::Amount;
use crate::DateTime;
use crate::Id;
use async_graphql::Enum;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::sql_types::Text;
use serde::Deserialize;
use serde::Serialize;

// # Types

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RentStatus {
    Partial,
    Pending,
    Settled,
}

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

// # Impls

impl FromSql<Text, Pg> for RentStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match bytes {
            Some(b"PARTIAL") => Ok(RentStatus::Partial),
            Some(b"PENDING") => Ok(RentStatus::Pending),
            Some(b"SETTLED") => Ok(RentStatus::Settled),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
