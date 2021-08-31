use async_graphql::scalar;
use piteo_core::Amount;
use serde::Deserialize;
use serde::Serialize;

// # Scalars. https://async-graphql.github.io/async-graphql/en/custom_scalars.html

scalar!(AuthId, "AuthenticationID");

scalar!(Date);

scalar!(DateTime);

scalar!(Decimal);

scalar!(Email);

scalar!(PhoneNumber);

scalar!(Url, "URL");

#[derive(Serialize, Deserialize)]
pub struct AuthId(String);

impl From<piteo_core::AuthId> for AuthId {
    fn from(item: piteo_core::AuthId) -> Self {
        Self(item.inner().to_string())
    }
}

#[derive(Serialize, Deserialize)]
pub struct Date(piteo_core::DateTime);

#[derive(Serialize, Deserialize)]
pub struct DateTime(piteo_core::DateTime);

impl From<piteo_core::DateTime> for DateTime {
    fn from(item: piteo_core::DateTime) -> Self {
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

impl From<String> for Email {
    fn from(item: String) -> Self {
        Self(item)
    }
}

#[derive(Serialize, Deserialize)]
pub struct PhoneNumber(String);

impl From<String> for PhoneNumber {
    fn from(item: String) -> Self {
        Self(item)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Url(String);
