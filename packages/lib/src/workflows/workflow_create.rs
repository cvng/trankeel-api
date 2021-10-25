use crate::error::Result;
use trankeel_core::database::Db;
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

pub fn create_workflow(db: &impl Db, input: CreateWorkflowInput) -> Result<Workflow> {
    input.validate()?;

    let candidacy = db.candidacies().by_id(&input.workflowable_id)?; // TODO: match workflowable
    let workflowable = db
        .workflowables()
        .create(Workflowable::Candidacy(candidacy))?;

    let workflow = db.workflows().create(Workflow {
        id: WorkflowId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        workflowable_id: workflowable.id(),
        type_: input.type_,
        completed: Default::default(),
    })?;

    workflow
        .steps()
        .into_iter()
        .map(|step| db.steps().create(step))
        .collect::<Result<Vec<_>>>()?;

    Ok(workflow)
}
