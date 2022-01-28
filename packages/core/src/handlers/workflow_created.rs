use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::WorkflowCreated;

pub fn workflow_created(ctx: &Context, event: WorkflowCreated) -> Result<()> {
    let db = ctx.db();

    let WorkflowCreated {
        workflow,
        workflowable,
    } = event;

    db.workflowables().create(&workflowable)?;
    db.workflows().create(&workflow)?;

    Ok(())
}
