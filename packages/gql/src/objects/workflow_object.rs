use trankeel::DateTime;
use trankeel::StepId;
use trankeel::WorkflowId;
use trankeel::WorkflowType;
use trankeel::WorkflowWithSteps;

#[derive(SimpleObject)]
pub struct Step {
    pub id: StepId,
    pub label: String,
    pub completed: bool,
    pub completed_at: Option<DateTime>,
}

impl From<trankeel::Step> for Step {
    fn from(item: trankeel::Step) -> Self {
        Self {
            id: item.id,
            label: item.label,
            completed: item.completed,
            completed_at: item.updated_at,
        }
    }
}

#[derive(SimpleObject)]
pub struct Workflow {
    pub id: WorkflowId,
    pub type_: WorkflowType,
    pub steps: Vec<Step>,
}

impl From<WorkflowWithSteps> for Workflow {
    fn from(item: WorkflowWithSteps) -> Self {
        Self {
            id: item.0.id,
            type_: item.0.type_,
            steps: item.1.into_iter().map(Into::into).collect(),
        }
    }
}
