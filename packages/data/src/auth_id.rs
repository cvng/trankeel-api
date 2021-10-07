#[derive(Clone, Debug, Serialize, Deserialize, DieselNewType)]
pub struct AuthId(String);

impl AuthId {
    pub fn new(auth_id: String) -> Self {
        Self(auth_id)
    }

    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl Default for AuthId {
    fn default() -> Self {
        Self(Default::default())
    }
}

scalar!(AuthId, "AuthenticationID");
