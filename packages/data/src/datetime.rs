use chrono::TimeZone;
use chrono::Utc;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, DieselNewType)]
pub struct Date(pub chrono::NaiveDate);

impl From<chrono::NaiveDate> for Date {
    fn from(item: chrono::NaiveDate) -> Self {
        Self(item)
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize, DieselNewType)]
pub struct DateTime(pub chrono::DateTime<Utc>);

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
