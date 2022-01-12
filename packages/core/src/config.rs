use serde::Deserialize;
use serde_json::to_string;
use serde_json::Value;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::path::Path;
use trankeel_data::Candidacy;
use trankeel_data::Email;
use trankeel_data::Invite;
use trankeel_data::StepId;
use trankeel_data::Url;

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

impl From<Requirement> for trankeel_data::Requirement {
    fn from(item: Requirement) -> Self {
        Self {
            name: item.name,
            type_: item.type_.into(),
            value: None,
        }
    }
}

pub fn config() -> Config {
    toml::from_str::<Config>(CONFIG).unwrap()
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

pub fn candidacy_url(candidacy: &Candidacy) -> Url {
    config()
        .routes("candidacy_url")
        .unwrap()
        .replace(":id", &candidacy.id.to_string())
        .into()
}

pub fn invite_url(invite: &Invite, email: Email) -> Url {
    config()
        .routes("invite_url")
        .unwrap()
        .replace(":token", invite.token.inner())
        .replace(":email", email.inner())
        .into()
}

pub fn workflow_steps(workflow: &trankeel_data::Workflow) -> Vec<trankeel_data::Step> {
    config()
        .workflows(&to_string(&workflow.type_).unwrap())
        .unwrap()
        .parse()
        .into_iter()
        .map(|step| trankeel_data::Step {
            id: StepId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            workflow_id: workflow.id,
            label: step.label,
            event: Some(step.event),
            completed: Default::default(),
            confirmation: Some(step.confirmation),
            requirements: step
                .requirements
                .map(|requirements| trankeel_data::RequirementOuter {
                    requirements: requirements.into_iter().map(Into::into).collect(),
                }),
        })
        .collect()
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
