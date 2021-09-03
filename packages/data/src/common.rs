use serde::Deserialize;
use serde::Serialize;

pub type Id = uuid::Uuid;

pub type Amount = rust_decimal::Decimal;

#[derive(DieselNewType, Debug, Copy, Clone, Serialize, Deserialize)]
pub struct DateTime(chrono::NaiveDateTime);

impl DateTime {
    pub fn inner(&self) -> chrono::NaiveDateTime {
        self.0
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
