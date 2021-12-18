use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use crate::messenger::Messenger;
use crate::templates;
use trankeel_data::locale;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::Lease;
use trankeel_data::Name;
use trankeel_data::Person;
use trankeel_data::Step;
use trankeel_data::StepId;
use trankeel_ops::workflows::CompleteStep;
use trankeel_ops::workflows::CompleteStepInput;
use trankeel_ops::workflows::CompleteStepPayload;
use trankeel_ops::workflows::CompleteStepRequirementInput;
use trankeel_ops::Command;

#[derive(Clone)]
pub struct StepCompletedRequirement {
    pub name: String,
    pub value: String,
}

#[derive(Clone)]
pub struct StepCompleted {
    pub step_id: StepId,
    pub requirements: Option<Vec<StepCompletedRequirement>>,
}

impl From<StepCompleted> for Event {
    fn from(item: StepCompleted) -> Self {
        Self::StepCompleted(item)
    }
}

pub fn step_completed(ctx: &Context, event: StepCompleted) -> Result<()> {
    let db = ctx.db();

    let StepCompleted {
        step_id,
        requirements,
    } = event;

    let step = db.steps().by_id(&step_id)?;
    let participant = db.persons().by_step_id(&step.id)?;
    let lease = db.leases().by_person_id(&participant.id)?;
    let discussion = db.discussions().by_initiator_id(&participant.id)?;

    let CompleteStepPayload {
        step,
        lease,
        discussion,
    } = CompleteStep::new(&step, &lease, &discussion).run(CompleteStepInput {
        id: step.id,
        requirements: requirements.map(|requirements| {
            requirements
                .into_iter()
                .map(|requirement| CompleteStepRequirementInput {
                    name: requirement.name,
                    value: requirement.value,
                })
                .collect()
        }),
    })?;

    db.steps().update(&step)?;
    db.leases().update(&lease)?;
    db.discussions().update(&discussion)?;

    Ok(())
}

pub async fn step_completed_async(ctx: &Context, event: StepCompleted) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let StepCompleted { step_id, .. } = event;

    let step = db.steps().by_id(&step_id)?;
    let account = db.accounts().by_step_id(&step.id)?;
    let participant = db.persons().by_step_id(&step.id)?;
    let sender = db.persons().by_account_id_first(&account.id)?;
    let tenant = db.tenants().by_person_id(&participant.id)?;
    let lease = db.leases().by_tenant_id(&tenant.id)?; // TODO: match workflowable

    messenger.message(
        EventType::StepCompleted,
        Eventable::Step(step.clone()),
        sender.id,
        participant.id,
        render_step_message(&step, &participant, &lease)?,
    )?;

    Ok(())
}

fn render_step_message(step: &Step, participant: &Person, lease: &Lease) -> Result<Option<String>> {
    let message = step.confirmation.clone().map(|message| {
        templates::parse_template(&message)
            .unwrap()
            .render(&liquid::object!({
                "tenant_name": participant.display_name(),
                "effect_date": lease.effect_date.inner().to_rfc3339(),
                "deposit_amount": locale::currency(&lease.deposit_amount.inner().round_dp(2).to_string()),
            }))
            .unwrap()
    });

    Ok(message)
}
