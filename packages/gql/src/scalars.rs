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

impl From<AuthId> for piteo_core::AuthId {
    fn from(item: AuthId) -> Self {
        Self::new(item.0)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Date(piteo_core::DateTime);

impl From<Date> for piteo_core::DateTime {
    fn from(item: Date) -> Self {
        item.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct DateTime(piteo_core::DateTime);

impl From<piteo_core::DateTime> for DateTime {
    fn from(item: piteo_core::DateTime) -> Self {
        Self(item)
    }
}

impl From<DateTime> for piteo_core::DateTime {
    fn from(item: DateTime) -> Self {
        item.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct Decimal(Amount);

impl From<Amount> for Decimal {
    fn from(item: Amount) -> Self {
        Self(item)
    }
}

impl From<Decimal> for Amount {
    fn from(item: Decimal) -> Self {
        item.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct Email(String);

impl From<String> for Email {
    fn from(item: String) -> Self {
        Self(item)
    }
}

impl From<Email> for String {
    fn from(item: Email) -> Self {
        item.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct PhoneNumber(String);

impl From<String> for PhoneNumber {
    fn from(item: String) -> Self {
        Self(item)
    }
}

impl From<PhoneNumber> for String {
    fn from(item: PhoneNumber) -> Self {
        item.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct Url(String);
