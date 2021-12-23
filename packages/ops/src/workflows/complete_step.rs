use crate::error::Result;
use crate::Command;
use trankeel_core::dispatcher::Event;
use trankeel_core::handlers::StepCompleted;
use trankeel_core::handlers::StepCompletedRequirement;
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

pub struct CompleteStepCommand {
    step: Step,
}

impl CompleteStepCommand {
    pub fn new(step: &Step) -> Self {
        Self { step: step.clone() }
    }
}

impl Command for CompleteStepCommand {
    type Input = CompleteStepInput;
    type Payload = Vec<Event>;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self { step } = self;

        let events = vec![StepCompleted::with(
            step.id,
            input.requirements.map(|requirements| {
                requirements
                    .into_iter()
                    .map(|requirement| StepCompletedRequirement {
                        name: requirement.name,
                        value: requirement.value,
                    })
                    .collect()
            }),
        )];

        // TODO: Dispatch step event (ex: "candidacy_accepted“).
        if let Some(_step_event) = step.event {}

        Ok(events)
    }
}
