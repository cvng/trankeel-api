use serde::Deserialize;
use serde::Serialize;

pub type Id = uuid::Uuid;

pub type Amount = rust_decimal::Decimal;

pub type Email = String;

pub type PhoneNumber = String;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, DieselNewType)]
pub struct DateTime(chrono::NaiveDateTime);

impl DateTime {
    pub fn inner(&self) -> chrono::NaiveDateTime {
        self.0
    }

    pub fn from_timestamp(secs: i64, nsecs: u32) -> Self {
        Self(chrono::NaiveDateTime::from_timestamp(secs, nsecs))
    }
}

impl Default for DateTime {
    fn default() -> Self {
        Self(chrono::NaiveDateTime::from_timestamp(0, 0))
    }
}

pub trait LegalEntity {}

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
