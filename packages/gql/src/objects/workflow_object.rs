use super::Step;
use async_graphql::SimpleObject;
use trankeel::WorkflowId;
use trankeel::WorkflowType;
use trankeel::WorkflowWithSteps;

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

impl From<trankeel::Workflow> for Workflow {
    fn from(item: trankeel::Workflow) -> Self {
        Self {
            id: item.id,
            type_: item.type_,
            steps: Vec::new(), // TODO: Workflow = graphql(complex)
        }
    }
}
