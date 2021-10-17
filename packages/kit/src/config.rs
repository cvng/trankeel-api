use serde::Deserialize;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io;

const CONFIG: &str = include_str!("../../../trankeel.toml");

#[derive(Deserialize)]
pub struct Config {
    templates: BTreeMap<String, Template>,
    routes: BTreeMap<String, String>,
}

impl Config {
    pub fn templates(&self, key: &str) -> Option<Template> {
        self.templates.get(key).cloned()
    }

    pub fn web_routes(&self, key: &str) -> Option<String> {
        self.routes.get(key).cloned()
    }
}

#[derive(Clone, Deserialize)]
pub struct Template {
    pub id: String,
    pub path: String,
}

impl Template {
    pub fn as_string(self) -> Result<String, io::Error> {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        fs::read_to_string(format!("{}/../../{}", manifest_dir, self.path))
    }
}

pub fn config() -> Config {
    toml::from_str::<Config>(CONFIG).unwrap()
}
