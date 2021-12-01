use crate::schema::workflows;
use crate::DateTime;
use crate::Id;
use crate::Step;
use crate::WorkflowableId;

pub type WorkflowId = Id;

pub type WorkflowWithSteps = (Workflow, Vec<Step>);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DbEnum, Enum)]
#[DieselType = "Workflowtype"]
#[serde(rename_all = "snake_case")]
pub enum WorkflowType {
    Candidacy,
}

#[derive(Clone, Debug, Identifiable, Insertable, Queryable)]
pub struct Workflow {
    pub id: WorkflowId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub workflowable_id: WorkflowableId,
    pub type_: WorkflowType,
    pub completed: bool,
}

#[derive(Default, AsChangeset, Identifiable, Insertable)]
#[table_name = "workflows"]
pub struct WorkflowData {
    pub id: WorkflowId,
    pub workflowable_id: Option<WorkflowableId>,
    pub type_: Option<WorkflowType>,
    pub completed: Option<bool>,
}
