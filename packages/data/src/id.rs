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

scalar!(Id, "ID");
