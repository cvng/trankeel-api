use async_graphql::InputValueError;
use async_graphql::InputValueResult;
use async_graphql::ScalarType;
use async_graphql::Value;
use std::fmt;
use std::fmt::Display;

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, Default, DieselNewType,
)]
pub struct Id(uuid::Uuid);

impl Id {
    pub fn new_v4() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[async_graphql::Scalar(name = "ID")]
impl ScalarType for Id {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(value) => Ok(uuid::Uuid::parse_str(&value).map(Id)?),
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
