use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::sql_types::Text;

pub enum RentStatus {
    Partial,
    Pending,
    Settled,
}

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

// # Models

#[derive(Queryable)]
pub struct Rent {
    pub id: uuid::Uuid,
    pub period_end: chrono::NaiveDateTime,
    pub period_start: chrono::NaiveDateTime,
    pub amount: decimal::Decimal,
    pub charges_amount: Option<decimal::Decimal>,
    pub full_amount: decimal::Decimal,
    pub status: RentStatus,
    pub account_id: uuid::Uuid,
    pub lease_id: uuid::Uuid,
    pub receipt_id: Option<uuid::Uuid>,
    pub transaction_id: Option<uuid::Uuid>,
    pub notice_id: Option<uuid::Uuid>,
}
