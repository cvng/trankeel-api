use async_graphql::scalar;
use piteo_core::chrono;
use piteo_core::Amount;
use serde::Deserialize;
use serde::Serialize;

// # Scalars. https://async-graphql.github.io/async-graphql/en/custom_scalars.html

scalar!(AuthId, "AuthID");

scalar!(DateTime);

scalar!(Decimal);

scalar!(Email);

scalar!(PhoneNumber);

#[derive(Serialize, Deserialize)]
pub struct AuthId(String);

impl From<piteo_core::AuthId> for AuthId {
    fn from(item: piteo_core::AuthId) -> Self {
        Self(item.inner().to_string())
    }
}

#[derive(Serialize, Deserialize)]
pub struct DateTime(chrono::NaiveDateTime);

impl From<chrono::NaiveDateTime> for DateTime {
    fn from(item: chrono::NaiveDateTime) -> Self {
        Self(item)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Decimal(Amount);

impl From<Amount> for Decimal {
    fn from(item: Amount) -> Self {
        Self(item)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Email(String);

#[derive(Serialize, Deserialize)]
pub struct PhoneNumber(String);
