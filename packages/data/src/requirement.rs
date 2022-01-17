use trankeel_kit::config;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum RequirementType {
    Date,
}

impl From<String> for RequirementType {
    fn from(item: String) -> Self {
        match item.as_str() {
            "date" => Self::Date,
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, AsJsonb, SimpleObject)]
pub struct RequirementOuter {
    pub requirements: Vec<Requirement>,
}

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct Requirement {
    pub name: String,
    pub type_: RequirementType,
    pub value: Option<String>,
}

impl From<config::Requirement> for Requirement {
    fn from(item: config::Requirement) -> Self {
        Self {
            name: item.name,
            type_: item.type_.into(),
            value: None,
        }
    }
}
