use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Handler;
use crate::error::Result;
use crate::messenger::Messenger;
use crate::templates;
use chrono::DateTime;
use chrono::Utc;
use trankeel_data::Discussion;
use trankeel_data::DiscussionStatus;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::Lease;
use trankeel_data::Name;
use trankeel_data::Person;
use trankeel_data::Requirement;
use trankeel_data::RequirementOuter;
use trankeel_data::Step;
use trankeel_data::StepEvent;
use trankeel_kit::locale;
use trankeel_ops::event::StepCompleted;

pub struct StepCompletedPayload {
    pub step: Step,
    pub lease: Lease,
    pub discussion: Discussion,
}

pub struct StepCompletedHandler {
    pub step: Step,
    pub lease: Lease,
    pub discussion: Discussion,
}

impl StepCompletedHandler {
    pub fn new(step: &Step, lease: &Lease, discussion: &Discussion) -> Self {
        Self {
            step: step.clone(),
            lease: lease.clone(),
            discussion: discussion.clone(),
        }
    }
}

impl Handler for StepCompletedHandler {
    type Event = StepCompleted;
    type Payload = StepCompletedPayload;

    fn run(self, event: Self::Event) -> Result<Self::Payload> {
        let Self {
            step,
            lease,
            discussion,
        } = self;

        let step = Step {
            id: step.id,
            completed: !step.completed,
            requirements: match_requirements(&step, event),
            ..step
        };

        let (lease, discussion) = if let Some(step_event) = step.event.clone() {
            match step_event.into() {
                StepEvent::LeaseSigned => {
                    let lease = Lease {
                        signature_date: Some(Utc::now().into()), // TODO: match workflowable
                        ..lease
                    };
                    (lease, discussion)
                }
                StepEvent::LeaseConfirmed => {
                    let step_requirements = match step.clone().requirements {
                        Some(step_requirements) => step_requirements.requirements,
                        None => vec![],
                    };

                    let effect_date = step_requirements
                        .into_iter()
                        .find(|sr| sr.name == "effect_date")
                        .and_then(|sr| sr.value);

                    if let Some(effect_date) = effect_date {
                        let lease = Lease {
                            effect_date: effect_date.parse::<DateTime<Utc>>()?.into(), // TODO: match workflowable
                            ..lease
                        };
                        (lease, discussion)
                    } else {
                        (lease, discussion)
                    }
                }
                StepEvent::LeaseActivated => {
                    // Make discussion active now.
                    let discussion = Discussion {
                        status: DiscussionStatus::Active,
                        ..discussion
                    };
                    (lease, discussion)
                }
                _ => (lease, discussion),
            }
        } else {
            (lease, discussion)
        };

        Ok(Self::Payload {
            step,
            lease,
            discussion,
        })
    }
}

pub fn step_completed(ctx: &Context, event: StepCompleted) -> Result<()> {
    let db = ctx.db();

    let step = db.steps().by_id(&event.step_id)?;
    let participant = db.persons().by_step_id(&step.id)?;
    let lease = db.leases().by_person_id(&participant.id)?;
    let discussion = db.discussions().by_initiator_id(&participant.id)?;

    StepCompletedHandler::new(&step, &lease, &discussion)
        .run(event)
        .and_then(|payload| {
            db.steps().update(&payload.step)?;
            db.leases().update(&payload.lease)?;
            db.discussions().update(&payload.discussion)?;
            Ok(())
        })
}

pub async fn step_completed_async(ctx: &Context, event: StepCompleted) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let step = db.steps().by_id(&event.step_id)?;
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

// # Utils

fn match_requirements(step: &Step, event: StepCompleted) -> Option<RequirementOuter> {
    let requirements = match step.requirements.clone() {
        Some(requirements) => requirements.requirements,
        None => return None,
    };

    let requirements_input = match event.requirements {
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
