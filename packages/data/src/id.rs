use async_graphql::InputValueError;
use async_graphql::InputValueResult;
use async_graphql::ScalarType;
use async_graphql::Value;
use diesel_derive_newtype::DieselNewType;
use fake::Dummy;
use fake::Fake;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Default, DieselNewType, Dummy,
)]
pub struct Id(uuid::Uuid);

impl Id {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl FromStr for Id {
    type Err = uuid::Error;

    fn from_str(uuid_str: &str) -> Result<Self, Self::Err> {
        Ok(Self(uuid::Uuid::parse_str(uuid_str)?))
    }
}

#[async_graphql::Scalar(name = "ID")]
impl ScalarType for Id {
    fn parse(value: Value) -> InputValueResult<Self> {
        if let Value::String(value) = value {
            Ok(uuid::Uuid::parse_str(&value).map(Id)?)
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
