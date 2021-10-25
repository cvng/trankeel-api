use serde_json::from_str;

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Enum)]
pub enum RequirementType {
    Date,
}

#[derive(Clone, Debug, Serialize, Deserialize, AsJsonb, SimpleObject)]
pub struct RequirementOuter {
    pub requirements: Vec<Requirement>,
}

#[derive(Clone, Debug, Serialize, Deserialize, SimpleObject)]
pub struct Requirement {
    pub name: String,
    pub type_: RequirementType,
}

impl From<trankeel_kit::config::Requirement> for Requirement {
    fn from(item: trankeel_kit::config::Requirement) -> Self {
        Self {
            name: item.name,
            type_: from_str(&item.type_).unwrap(),
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
