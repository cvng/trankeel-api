use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use crate::messenger::Messenger;
use chrono::DateTime;
use chrono::Utc;
use diesel::result::Error::NotFound;
use trankeel_data::locale;
use trankeel_data::Discussion;
use trankeel_data::DiscussionStatus;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::Lease;
use trankeel_data::Name;
use trankeel_data::Person;
use trankeel_data::Step;
use trankeel_data::StepEvent;

#[derive(Clone)]
pub struct StepCompleted {
    pub step: Step,
}

impl From<StepCompleted> for Event {
    fn from(item: StepCompleted) -> Self {
        Self::StepCompleted(item)
    }
}

pub fn step_completed(ctx: &Context, event: StepCompleted) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let StepCompleted { step } = event;

    let account = db.accounts().by_step_id(&step.id)?;
    let participant = db.persons().by_step_id(&step.id)?;
    let sender = db
        .persons()
        .by_account_id(&account.id)?
        .first()
        .cloned()
        .ok_or(NotFound)?;
    let eventable = db.eventables().create(&Eventable::Step(step.clone()))?;

    if let Some(step_event) = step.event.clone() {
        match step_event.into() {
            StepEvent::LeaseSigned => {
                let lease = db.leases().by_person_id(&participant.id)?;
                db.leases().update(&Lease {
                    id: lease.id,
                    signature_date: Some(Utc::now().into()), // TODO: match workflowable
                    ..lease
                })?;
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
                    let lease = db.leases().by_person_id(&participant.id)?;
                    db.leases().update(&Lease {
                        id: lease.id,
                        effect_date: effect_date.parse::<DateTime<Utc>>()?.into(), // TODO: match workflowable
                        ..lease
                    })?;
                }
            }
            StepEvent::LeaseActivated => {
                let discussion = db.discussions().by_initiator_id(&participant.id)?;
                db.discussions().update(&Discussion {
                    id: discussion.id,
                    status: DiscussionStatus::Active,
                    ..discussion
                })?;
            }
            _ => (),
        }
    }

    let tenant = db.tenants().by_person_id(&participant.id)?;
    let lease = db.leases().by_tenant_id(&tenant.id)?; // TODO: match workflowable
    let message = render_step_message(step, participant.clone(), lease)?;

    messenger.message(
        db,
        EventType::StepCompleted,
        eventable.id(),
        account.id,
        sender.id,
        participant.id,
        message,
    )?;

    Ok(())
}

// # Utils

fn render_step_message(step: Step, participant: Person, lease: Lease) -> Result<Option<String>> {
    let message = step.confirmation.map(|message| {
        message
            .replace("{{ tenant_name }}", &participant.display_name())
            .replace("{{ effect_date }}", &lease.effect_date.inner().to_rfc3339())
            .replace(
                "{{ deposit_amount }}",
                &locale::currency(&lease.deposit_amount.inner().round_dp(2).to_string()),
            )
    });

    Ok(message)
}
