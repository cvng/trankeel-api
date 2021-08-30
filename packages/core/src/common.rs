use crate::locale;
use diesel::deserialize;
use diesel::deserialize::FromSql;
use diesel::pg::Pg;
use diesel::sql_types::Jsonb;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fmt::Display;

pub trait Name {
    fn first_name(&self) -> String;

    fn last_name(&self) -> String;

    fn full_name(&self) -> String {
        self.display_name()
    }

    fn short_name(&self) -> String {
        self.display_name()
    }

    fn display_name(&self) -> String {
        [&self.first_name(), &self.last_name()]
            .iter()
            .map(|&v| v.clone())
            .collect::<Vec<String>>()
            .join(" ")
            .trim()
            .to_string()
    }
}

pub trait LegalEntity {}

pub type Id = rust_uuid::Uuid;

pub type Amount = rust_decimal::Decimal;

pub type DateTime = rust_chrono::NaiveDateTime;

#[derive(Clone, Serialize, Deserialize, Debug, FromSqlRow)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub city: Option<String>,
    pub country: Option<String>,
    pub line1: Option<String>,
    pub line2: Option<String>,
    pub postal_code: Option<String>,
}

impl Address {
    pub fn inline(&self) -> String {
        [
            &self.line1,
            &self.line2,
            &self.postal_code,
            &self.city,
            &self.country,
        ]
        .iter()
        .filter_map(|&v| v.clone())
        .collect::<Vec<String>>()
        .join(", ")
        .trim()
        .to_string()
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inline())
    }
}

impl Default for Address {
    fn default() -> Self {
        Self {
            city: Default::default(),
            country: Some(locale::DEFAULT_COUNTRY.into()),
            line1: Default::default(),
            line2: Default::default(),
            postal_code: Default::default(),
        }
    }
}

impl FromSql<Jsonb, Pg> for Address {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let value = <serde_json::Value as FromSql<Jsonb, Pg>>::from_sql(bytes)?;
        Ok(serde_json::from_value(value)?)
    }
}
