use async_graphql::Enum;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::sql_types::Text;
use serde::Deserialize;
use serde::Serialize;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
