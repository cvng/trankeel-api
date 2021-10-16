use std::collections::hash_map::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Clone, Debug, Serialize, Deserialize, DieselNewType, Hash)]
pub struct InviteToken(String);

impl InviteToken {
    pub fn new(s: String) -> Self {
        Self(calculate_hash(&s))
    }

    pub fn inner(&self) -> &str {
        &self.0
    }
}

scalar!(InviteToken);

fn calculate_hash(t: &impl Hash) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish().to_string()
}
