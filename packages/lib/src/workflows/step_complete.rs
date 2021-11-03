use crate::error::Result;
use trankeel_core::activity::trace;
use trankeel_core::activity::Trace;
use trankeel_core::database::Db;
use trankeel_data::RequirementOuter;
use trankeel_data::Step;
use trankeel_data::StepData;
use trankeel_data::StepId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CompleteStepInput {
    pub id: StepId,
    pub requirements: Option<Vec<RequirementInput>>,
}

#[derive(Clone, InputObject, Validate)]
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
        requirements: map_requirements(step, input),
        ..Default::default()
    })?;

    trace(vec![Trace::StepCompleted(step.clone())]).ok();

    Ok(step)
}

fn map_requirements(step: Step, input: CompleteStepInput) -> Option<RequirementOuter> {
    let step_requirements = match step.requirements {
        Some(step_requirements) => step_requirements.requirements,
        None => return None,
    };

    let input_requirements = match input.requirements {
        Some(input_requirements) => input_requirements,
        None => return None,
    };

    let requirements = step_requirements
        .into_iter()
        .map(|mut sr| {
            sr.value = input_requirements
                .clone()
                .into_iter()
                .find(|ir| ir.name == sr.name)
                .map(|ir| ir.value);
            sr
        })
        .collect();

    Some(RequirementOuter { requirements })
}
