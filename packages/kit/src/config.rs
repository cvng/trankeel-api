use serde::Deserialize;
use std::collections::BTreeMap;

const CONFIG: &str = include_str!("../../../piteo.toml");

#[derive(Deserialize)]
pub struct Config {
    templates: BTreeMap<String, String>,
}

impl Config {
    pub fn templates(&self, key: &str) -> Option<&String> {
        self.templates.get(key)
    }
}

pub fn config() -> Config {
    toml::from_str::<Config>(CONFIG).unwrap()
}
