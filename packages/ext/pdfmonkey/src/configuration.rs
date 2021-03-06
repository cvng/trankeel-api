use crate::Pdfmonkey;
use std::env;

#[derive(Clone)]
pub struct Configuration {
    pub host: String,
    pub namespace: String,
    pub private_key: String,
}

impl Configuration {
    pub fn new() -> Self {
        Self {
            host: "https://api.pdfmonkey.io".into(),
            namespace: "api/v1".into(),
            private_key: env::var("PDFMONKEY_PRIVATE_KEY").expect("PDFMONKEY_PRIVATE_KEY"),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self::new()
    }
}

impl Pdfmonkey {
    pub fn new() -> Self {
        Self {
            config: Configuration::default(),
        }
    }

    pub fn configure(config: Configuration) -> Self {
        Self { config }
    }

    pub(crate) fn configuration(&self) -> Configuration {
        self.config.clone()
    }
}
