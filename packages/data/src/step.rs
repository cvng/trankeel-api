use crate::schema::steps;
use crate::DateTime;
use crate::Id;
use crate::RequirementOuter;
use crate::Workflow;
use crate::WorkflowId;

pub type StepId = Id;

#[derive(Clone, Debug, Insertable, Queryable, Identifiable, Associations, SimpleObject)]
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
}

#[derive(Default, AsChangeset, Identifiable, Insertable)]
#[table_name = "steps"]
pub struct StepData {
    pub id: StepId,
    pub workflow_id: Option<WorkflowId>,
    pub label: Option<String>,
    pub completed: Option<bool>,
    pub confirmation: Option<String>,
}
