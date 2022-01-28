use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::StepCreated;

pub fn step_created(ctx: &Context, event: StepCreated) -> Result<()> {
    let db = ctx.db();

    let StepCreated { step } = event;

    db.steps().create(&step)?;

    Ok(())
}
