use crate::error::Result;
use trankeel_core::activity::trace;
use trankeel_core::activity::Trace;
use trankeel_core::database::Db;
use trankeel_data::Step;
use trankeel_data::StepData;
use trankeel_data::StepId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CompleteStepInput {
    pub id: StepId,
    pub requirements: Option<Vec<RequirementInput>>,
}

#[derive(InputObject, Validate)]
pub struct RequirementInput {
    pub name: String,
    pub value: String,
}

pub fn complete_step(db: &impl Db, input: CompleteStepInput) -> Result<Step> {
    input.validate()?;

    let step = db.steps().by_id(&input.id)?;

    let step = db.steps().update(StepData {
        id: step.id,
        completed: Some(!step.completed),
        ..Default::default()
    })?;

    trace(db, Trace::StepCompleted(step.clone())).ok();

    Ok(step)
}
