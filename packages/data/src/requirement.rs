#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Enum)]
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

impl From<trankeel_kit::config::Requirement> for Requirement {
    fn from(item: trankeel_kit::config::Requirement) -> Self {
        Self {
            name: item.name,
            type_: item.type_.into(),
            value: None,
        }
    }
}

impl From<Vec<trankeel_kit::config::Requirement>> for RequirementOuter {
    fn from(item: Vec<trankeel_kit::config::Requirement>) -> Self {
        Self {
            requirements: item.into_iter().map(Into::into).collect(),
        }
    }
}
