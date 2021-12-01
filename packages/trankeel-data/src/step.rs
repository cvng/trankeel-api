use crate::schema::steps;
use crate::DateTime;
use crate::Id;
use crate::RequirementOuter;
use crate::Workflow;
use crate::WorkflowId;

pub type StepId = Id;

pub enum StepEvent {
    LeaseSigned,
    LeaseConfirmed,
    LeaseActivated,
}

impl From<String> for StepEvent {
    fn from(item: String) -> Self {
        match item.as_str() {
            "lease_signed" => Self::LeaseSigned,
            "lease_confirmed" => Self::LeaseConfirmed,
            "lease_activated" => Self::LeaseActivated,
            _ => unimplemented!(),
        }
    }
}

#[derive(
    Clone, Debug, Associations, Identifiable, Insertable, Queryable, SimpleObject, AsChangeset,
)]
#[belongs_to(parent = "Workflow")]
pub struct Step {
    pub id: StepId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub workflow_id: WorkflowId,
    pub label: String,
    pub completed: bool,
    pub confirmation: Option<String>,
    pub requirements: Option<RequirementOuter>,
    pub event: Option<String>,
}
