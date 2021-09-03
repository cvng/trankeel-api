use async_graphql::Enum;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::sql_types::Text;
use serde::Deserialize;
use serde::Serialize;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LeaseStatus {
    Active,
    Ended,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, FromSqlRow, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LeaseType {
    Furnished,
    Naked,
}

impl FromSql<Text, Pg> for LeaseType {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match bytes {
            Some(b"FURNISHED") => Ok(LeaseType::Furnished),
            Some(b"NAKED") => Ok(LeaseType::Naked),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
