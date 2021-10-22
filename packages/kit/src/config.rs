use serde::Deserialize;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io;

const CONFIG: &str = include_str!("../../../trankeel.toml");

#[derive(Deserialize)]
pub struct Config {
    routes: BTreeMap<String, String>,
    templates: BTreeMap<String, Template>,
    workflows: BTreeMap<String, Workflow>,
}

impl Config {
    pub fn routes(&self, key: &str) -> Option<String> {
        self.routes
            .get(key)
            .cloned()
            .map(|route| format!("{}{}", env::var("WEB_URL").expect("WEB_URL"), route))
    }

    pub fn templates(&self, key: &str) -> Option<Template> {
        self.templates.get(key).cloned()
    }

    pub fn workflows(&self, key: &str) -> Option<Workflow> {
        self.workflows.get(&key.replace('"', "")).cloned()
    }
}

#[derive(Clone, Deserialize)]
pub struct Template {
    pub id: String,
    pub path: String,
}

impl Template {
    pub fn as_string(&self) -> Result<String, io::Error> {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        fs::read_to_string(format!("{}/../../{}", manifest_dir, self.path))
    }
}

#[derive(Clone, Deserialize)]
pub struct Workflow {
    pub path: String,
}

impl Workflow {
    pub fn as_string(&self) -> Result<String, io::Error> {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        fs::read_to_string(format!("{}/../../{}", manifest_dir, self.path))
    }

    pub fn parse(&self) -> Vec<Step> {
        toml::from_str::<StepOuter>(&self.as_string().unwrap())
            .unwrap()
            .steps
    }
}

#[derive(Clone, Deserialize)]
pub struct StepOuter {
    pub steps: Vec<Step>,
}

#[derive(Clone, Deserialize)]
pub struct Step {
    pub label: String,
    pub confirmation: String,
}

pub fn config() -> Config {
    toml::from_str::<Config>(CONFIG).unwrap()
}
