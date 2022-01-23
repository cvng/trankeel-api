use async_graphql::scalar;
use chrono::TimeZone;
use chrono::Utc;
use diesel_derive_newtype::DieselNewType;
use fake::Dummy;
use fake::Fake;
use serde::Deserialize;
use serde::Serialize;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, DieselNewType)]
pub struct Date(chrono::NaiveDate);

impl From<chrono::NaiveDate> for Date {
    fn from(item: chrono::NaiveDate) -> Self {
        Self(item)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, DieselNewType, Dummy)]
pub struct DateTime(chrono::DateTime<Utc>);

impl DateTime {
    pub fn new(secs: i64) -> Self {
        Self(Utc.timestamp(secs, 0))
    }

    pub fn inner(&self) -> chrono::DateTime<Utc> {
        self.0
    }
}

impl Default for DateTime {
    fn default() -> Self {
        Self::new(0)
    }
}

impl From<chrono::DateTime<Utc>> for DateTime {
    fn from(item: chrono::DateTime<Utc>) -> Self {
        Self(item)
    }
}

scalar!(Date);

scalar!(DateTime);
