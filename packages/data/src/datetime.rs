use async_graphql::scalar;
use chrono::Datelike;
use chrono::NaiveDate;
use chrono::TimeZone;
use chrono::Utc;
use chronoutil::delta;
use serde::Deserialize;
use serde::Serialize;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, DieselNewType)]
pub struct DateTime(chrono::DateTime<Utc>);

impl DateTime {
    pub fn from_timestamp(secs: i64, nsecs: u32) -> Self {
        Self(Utc.timestamp(secs, nsecs))
    }

    pub fn inner(&self) -> chrono::DateTime<Utc> {
        self.0
    }

    /// Shift a date by the given number of months. https://docs.rs/chronoutil/0.2.3
    pub fn shift_months(&self, months: i32) -> Self {
        delta::shift_months(self.inner(), months).into()
    }

    /// Returns the total number of days in the duration.
    pub fn duration_num_days(&self, rhs: Self) -> i64 {
        self.inner().signed_duration_since(rhs.inner()).num_days() + 1
    }

    /// Returns the total number of days in the current month.
    pub fn month_num_days(&self) -> i64 {
        let year = self.inner().year();
        let month = self.inner().month();
        NaiveDate::from_ymd(
            if month == 12 { year + 1 } else { year },
            if month == 12 { 1 } else { month + 1 },
            1,
        )
        .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
        .num_days()
    }

    pub fn last_day_of_month(&self) -> Self {
        let year = self.inner().year();
        let month = self.inner().month();
        let day = self.month_num_days() as u32;
        Self(chrono::DateTime::from_utc(
            chrono::NaiveDate::from_ymd(year, month, day).and_hms_milli(23, 59, 59, 999),
            Utc,
        ))
    }
}

impl Default for DateTime {
    fn default() -> Self {
        Self::from_timestamp(0, 0)
    }
}

impl From<chrono::DateTime<Utc>> for DateTime {
    fn from(item: chrono::DateTime<Utc>) -> Self {
        Self(item)
    }
}

scalar!(DateTime);
