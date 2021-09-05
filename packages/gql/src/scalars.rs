use async_graphql::scalar;
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

impl From<piteo::AuthId> for AuthId {
    fn from(item: piteo::AuthId) -> Self {
        Self(item.inner().to_string())
    }
}

impl From<AuthId> for piteo::AuthId {
    fn from(item: AuthId) -> Self {
        Self::new(item.0)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Date(piteo::DateTime);

impl From<Date> for piteo::DateTime {
    fn from(item: Date) -> Self {
        item.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct DateTime(piteo::DateTime);

impl From<piteo::DateTime> for DateTime {
    fn from(item: piteo::DateTime) -> Self {
        Self(item)
    }
}

impl From<DateTime> for piteo::DateTime {
    fn from(item: DateTime) -> Self {
        item.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct Decimal(piteo::Amount);

impl From<piteo::Amount> for Decimal {
    fn from(item: piteo::Amount) -> Self {
        Self(item)
    }
}

impl From<Decimal> for piteo::Amount {
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
