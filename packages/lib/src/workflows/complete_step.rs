use crate::error::Result;
use trankeel_core::dispatcher::Command;
use trankeel_data::Requirement;
use trankeel_data::RequirementOuter;
use trankeel_data::Step;
use trankeel_data::StepId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CompleteStepRequirementInput {
    pub name: String,
    pub value: String,
}

#[derive(InputObject, Validate)]
pub struct CompleteStepInput {
    pub id: StepId,
    pub requirements: Option<Vec<CompleteStepRequirementInput>>,
}

pub struct CompleteStepPayload {
    pub step: Step,
}

pub(crate) struct CompleteStep {
    step: Step,
}

impl CompleteStep {
    pub fn new(step: &Step) -> Self {
        Self { step: step.clone() }
    }
}

impl Command for CompleteStep {
    type Input = CompleteStepInput;
    type Payload = CompleteStepPayload;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let CompleteStep { step } = self;

        let step = Step {
            id: step.id,
            completed: !step.completed,
            requirements: match_requirements(step.clone(), input),
            ..step
        };

        Ok(CompleteStepPayload { step })
    }
}

fn match_requirements(step: Step, input: CompleteStepInput) -> Option<RequirementOuter> {
    let requirements = match step.requirements {
        Some(requirements) => requirements.requirements,
        None => return None,
    };

    let requirements_input = match input.requirements {
        Some(requirements_input) => requirements_input,
        None => return None,
    };

    let requirements = requirements
        .into_iter()
        .map(|requirement| Requirement {
            value: requirements_input
                .iter()
                .find(|input| input.name == requirement.name)
                .map(|input| input.value.clone()),
            ..requirement
        })
        .collect();

    Some(RequirementOuter { requirements })
}
