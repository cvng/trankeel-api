use async_graphql::scalar;
use diesel_derive_newtype::DieselNewType;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone, Default, Debug, Serialize, Deserialize, DieselNewType)]
pub struct AuthId(String);

impl AuthId {
    pub fn new(auth_id: String) -> Self {
        Self(auth_id)
    }

    pub fn inner(&self) -> &str {
        &self.0
    }
}

scalar!(AuthId, "AuthenticationID");
