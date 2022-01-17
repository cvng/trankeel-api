use colored_json::ToColoredJson;
use figment::providers::Env;
use figment::providers::Format;
use figment::providers::Toml;
use figment::Figment;
use regex::Regex;
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;

const CONFIG_FILE: &str = "trankeel.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    author: Option<String>,
    pub database_url: Option<String>,
    pub debug_auth_id: Option<String>,
    pub pdfmonkey_private_key: Option<String>,
    pub sendinblue_api_key: Option<String>,
    pub sentry_dsn: Option<String>,
    pub stripe_secret_key: Option<String>,
    pub stripe_default_price_id: Option<String>,
    pub web_url: Option<String>,
    routes: BTreeMap<String, String>,
    templates: BTreeMap<String, Template>,
    workflows: BTreeMap<String, Workflow>,
}

impl Config {
    pub fn new() -> Self {
        Figment::new()
            .merge(Toml::file(CONFIG_FILE))
            .merge(Env::prefixed(""))
            .extract()
            .unwrap()
    }

    pub fn author(&self) -> Author {
        author(self.author.clone().unwrap()).unwrap()
    }

    pub fn routes(&self, key: &str) -> Option<String> {
        self.routes
            .get(key)
            .cloned()
            .map(|route| format!("{}{}", self.web_url.clone().unwrap(), route))
    }

    pub fn templates(&self, key: &str) -> Option<Template> {
        self.templates.get(key).cloned()
    }

    pub fn workflows(&self, key: &str) -> Option<Workflow> {
        self.workflows.get(&key.replace('"', "")).cloned()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Template {
    pub id: String,
    pub path: String,
}

impl Template {
    pub fn as_string(&self) -> Result<String, io::Error> {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        fs::read_to_string(
            manifest_dir
                .parent()
                .unwrap()
                .parent()
                .unwrap()
                .join("templates")
                .join(&self.path),
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub event: String,
    pub confirmation: String,
    pub requirements: Option<Vec<Requirement>>,
}

#[derive(Clone, Deserialize)]
pub struct Requirement {
    pub name: String,
    #[serde(rename = "type")]
    pub type_: String,
}

pub fn config() -> Config {
    Config::default()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

fn author(text: String) -> Result<Author, regex::Error> {
    let caps = Regex::new(r"(?P<first_name>\w+) (?P<last_name>\w+) <(?P<email>.*)>")?
        .captures(&text)
        .ok_or_else(|| {
            regex::Error::Syntax("format: \"Dev TRANKEEL <hello@trankeel.dev>\"".into())
        })?;

    Ok(Author {
        first_name: caps["first_name"].into(),
        last_name: caps["last_name"].into(),
        email: caps["email"].into(),
    })
}

// # Utils

pub fn template_by_id(template_id: &str) -> Option<Template> {
    config()
        .templates
        .into_iter()
        .find_map(|(_key, template)| match template.id == template_id {
            true => Some(template),
            false => None,
        })
}

pub fn base_url() -> String {
    format!("http://localhost:{}", 8000)
}

pub fn write_json<P, T>(path: P, json: &T) -> io::Result<()>
where
    P: AsRef<Path>,
    T: serde::Serialize,
{
    let path = env::temp_dir().join(path);
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(path, &serde_json::to_string_pretty(&json)?)
}

pub fn read_json<P>(path: P) -> io::Result<Value>
where
    P: AsRef<Path>,
{
    let path = env::temp_dir().join(path);
    let file = File::open(path)?;
    Ok(serde_json::from_reader(file).unwrap())
}

pub fn log_json<P>(payload: &P)
where
    P: serde::Serialize,
{
    println!(
        "{}",
        serde_json::to_string(&payload)
            .unwrap()
            .to_colored_json_auto()
            .unwrap()
    );
}
