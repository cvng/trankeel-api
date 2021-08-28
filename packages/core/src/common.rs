use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::sql_types::Jsonb;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;

pub trait LegalEntity: Display {
    fn display_name(&self) -> String {
        self.to_string()
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, FromSqlRow)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub city: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub postal_code: Option<String>,
}

impl FromSql<Jsonb, Pg> for Address {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        Ok(serde_json::from_value(value)?)
    }
}
