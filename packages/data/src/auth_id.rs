#[derive(Clone, Debug, Default, Serialize, Deserialize, DieselNewType)]
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
