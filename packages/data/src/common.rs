use async_graphql::scalar;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fmt::Display;
use std::ops::Add;

pub(crate) type Id = uuid::Uuid;

pub type Email = String;

pub type PhoneNumber = String;

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, DieselNewType)]
pub struct Amount(rust_decimal::Decimal);

impl Amount {
    pub fn new(num: i64, scale: u32) -> Self {
        Self(rust_decimal::Decimal::new(num, scale))
    }

    pub fn inner(&self) -> rust_decimal::Decimal {
        self.0
    }
}

impl Add for Amount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

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

scalar!(Amount, "Decimal");

scalar!(DateTime);
