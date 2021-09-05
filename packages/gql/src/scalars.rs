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

impl From<piteo_lib::AuthId> for AuthId {
    fn from(item: piteo_lib::AuthId) -> Self {
        Self(item.inner().to_string())
    }
}

impl From<AuthId> for piteo_lib::AuthId {
    fn from(item: AuthId) -> Self {
        Self::new(item.0)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Date(piteo_lib::DateTime);

impl From<Date> for piteo_lib::DateTime {
    fn from(item: Date) -> Self {
        item.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct DateTime(piteo_lib::DateTime);

impl From<piteo_lib::DateTime> for DateTime {
    fn from(item: piteo_lib::DateTime) -> Self {
        Self(item)
    }
}

impl From<DateTime> for piteo_lib::DateTime {
    fn from(item: DateTime) -> Self {
        item.0
    }
}

#[derive(Serialize, Deserialize)]
pub struct Decimal(piteo_lib::Amount);

impl From<piteo_lib::Amount> for Decimal {
    fn from(item: piteo_lib::Amount) -> Self {
        Self(item)
    }
}

impl From<Decimal> for piteo_lib::Amount {
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
