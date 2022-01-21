use crate::error::Result;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::Workflow;
use trankeel_data::WorkflowId;
use trankeel_data::WorkflowType;
use trankeel_data::Workflowable;
use trankeel_data::WorkflowableId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateWorkflowInput {
    pub type_: WorkflowType,
    pub workflowable_id: WorkflowableId,
}

pub struct CreateWorkflowPayload {
    pub workflow: Workflow,
}

pub struct CreateWorkflow {
    workflowable: Workflowable,
}

impl CreateWorkflow {
    pub fn new(workflowable: &Workflowable) -> Self {
        Self {
            workflowable: workflowable.clone(),
        }
    }
}

impl Command for CreateWorkflow {
    type Input = CreateWorkflowInput;
    type Payload = CreateWorkflowPayload;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self { workflowable } = self;

        let workflow = Workflow {
            id: WorkflowId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            workflowable_id: workflowable.id(),
            type_: input.type_,
            completed: Default::default(),
        };

        Ok(Self::Payload { workflow })
    }
}
