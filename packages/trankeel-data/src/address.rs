use crate::locale;
use crate::Inline;
use std::fmt;
use std::fmt::Display;

// # Types

#[derive(Clone, Debug, Serialize, Deserialize, AsJsonb)]
#[serde(rename_all = "camelCase")]
pub struct Address {
    pub city: String,
    pub country: Option<String>,
    pub line1: String,
    pub line2: Option<String>,
    pub postal_code: String,
}

// # Impls

impl Inline for Address {
    fn city(&self) -> Option<String> {
        Some(self.city.clone())
    }

    fn country(&self) -> Option<String> {
        self.country.clone()
    }

    fn line1(&self) -> Option<String> {
        Some(self.line1.clone())
    }

    fn line2(&self) -> Option<String> {
        self.line2.clone()
    }

    fn postal_code(&self) -> Option<String> {
        Some(self.postal_code.clone())
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inline())
    }
}

impl Default for Address {
    fn default() -> Self {
        Self {
            city: Default::default(),
            country: Some(locale::DEFAULT_COUNTRY.into()),
            line1: Default::default(),
            line2: Default::default(),
            postal_code: Default::default(),
        }
    }
}
