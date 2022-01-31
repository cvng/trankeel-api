use fake::Fake;

#[macro_export]
macro_rules! id {
    ($t:ident) => {
        #[derive(
            Copy,
            Clone,
            Default,
            Debug,
            PartialEq,
            Eq,
            Hash,
            serde::Serialize,
            serde::Deserialize,
            diesel_derive_newtype::DieselNewType,
            fake::Dummy,
        )]
        pub struct $t(pub(crate) uuid::Uuid);

        impl $t {
            pub fn new() -> Self {
                Self(uuid::Uuid::new_v4())
            }
        }

        impl std::fmt::Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl std::str::FromStr for $t {
            type Err = uuid::Error;

            fn from_str(uuid_str: &str) -> std::result::Result<Self, Self::Err> {
                Ok(Self(uuid::Uuid::parse_str(uuid_str)?))
            }
        }

        #[async_graphql::Scalar(name = "ID")]
        impl async_graphql::ScalarType for $t {
            fn parse(value: async_graphql::Value) -> async_graphql::InputValueResult<Self> {
                if let async_graphql::Value::String(value) = value {
                    Ok(uuid::Uuid::parse_str(&value).map($t)?)
                } else {
                    Err(async_graphql::InputValueError::expected_type(value))
                }
            }

            fn to_value(&self) -> async_graphql::Value {
                async_graphql::Value::String(self.0.to_string())
            }
        }
    };
}

pub use id;

id!(Id);
