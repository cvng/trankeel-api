use crate::database::Db;
use crate::error::Result;
use crate::providers::Pg;
use chrono::DateTime;
use chrono::Utc;
use diesel::result::Error::NotFound;
use trankeel_data::AccountId;
use trankeel_data::Candidacy;
use trankeel_data::Discussion;
use trankeel_data::DiscussionStatus;
use trankeel_data::EventId;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::EventableId;
use trankeel_data::File;
use trankeel_data::Lease;
use trankeel_data::Message;
use trankeel_data::MessageId;
use trankeel_data::Name;
use trankeel_data::Notice;
use trankeel_data::Payment;
use trankeel_data::Person;
use trankeel_data::PersonId;
use trankeel_data::Receipt;
use trankeel_data::Step;
use trankeel_data::StepEvent;
use trankeel_kit::locale::DEFAULT_CURRENCY;

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum Event {
    CandidacyCreated(Candidacy),
    CandidacyAccepted(Candidacy),
    CandidacyRejected(Candidacy),
    LeaseCreated(Lease),
    PaymentCreated(Payment),
    NoticeCreated(File),
    NoticeSent(File),
    ReceiptCreated(File),
    ReceiptSent(File),
    StepCompleted(Step),
}

impl From<Event> for EventType {
    fn from(item: Event) -> Self {
        match item {
            Event::CandidacyCreated(_) => Self::CandidacyCreated,
            Event::CandidacyAccepted(_) => Self::CandidacyAccepted,
            Event::CandidacyRejected(_) => Self::CandidacyRejected,
            Event::LeaseCreated(_) => Self::LeaseCreated,
            Event::PaymentCreated(_) => Self::PaymentCreated,
            Event::NoticeCreated(_) => Self::NoticeCreated,
            Event::NoticeSent(_) => Self::NoticeSent,
            Event::ReceiptCreated(_) => Self::ReceiptCreated,
            Event::ReceiptSent(_) => Self::ReceiptSent,
            Event::StepCompleted(_) => Self::StepCompleted,
        }
    }
}

pub fn dispatch(events: Vec<Event>) -> Result<()> {
    let db = Pg::init();

    Pg::transaction(&db, || {
        events.iter().try_for_each(|event| match event {
            Event::CandidacyCreated(candidacy) => on_candidacy_created(&db, event, candidacy),
            Event::CandidacyAccepted(candidacy) => on_candidacy_accepted(&db, event, candidacy),
            Event::CandidacyRejected(candidacy) => on_candidacy_accepted(&db, event, candidacy),
            Event::LeaseCreated(payload) => on_lease_created(&db, event, payload),
            Event::PaymentCreated(payment) => on_payment_created(&db, event, payment),
            Event::NoticeCreated(notice) => on_notice_created(&db, event, notice),
            Event::NoticeSent(notice) => on_notice_created(&db, event, notice),
            Event::ReceiptCreated(receipt) => on_receipt_created(&db, event, receipt),
            Event::ReceiptSent(receipt) => on_receipt_created(&db, event, receipt),
            Event::StepCompleted(step) => on_step_completed(&db, event, step),
        })
    })
}

// # Handlers

fn on_candidacy_created(db: &Pg, event: &Event, candidacy: &Candidacy) -> Result<()> {
    let account = db.accounts().by_candidacy_id(&candidacy.id)?;
    let participant = db.persons().by_candidacy_id(&candidacy.id)?;
    let eventable = db
        .eventables()
        .create(&Eventable::Candidacy(candidacy.clone()))?;

    notify(
        db,
        event.clone().into(),
        eventable.id(),
        account.id,
        participant.id,
        participant.id,
        None,
    )?;

    Ok(())
}

fn on_candidacy_accepted(db: &Pg, event: &Event, candidacy: &Candidacy) -> Result<()> {
    let account = db.accounts().by_candidacy_id(&candidacy.id)?;
    let participant = db.persons().by_candidacy_id(&candidacy.id)?;
    let sender = db
        .persons()
        .by_account_id(&account.id)?
        .first()
        .cloned()
        .ok_or(NotFound)?;
    let eventable = Eventable::Candidacy(candidacy.clone()); // already created in on_candidacy_created

    notify(
        db,
        event.clone().into(),
        eventable.id(),
        account.id,
        sender.id,
        participant.id,
        None,
    )?;

    Ok(())
}

fn on_lease_created(db: &Pg, event: &Event, lease: &Lease) -> Result<()> {
    let account = db.accounts().by_lease_id(&lease.id)?;
    let participant = db.persons().by_lease_id(&lease.id)?;
    let sender = db
        .persons()
        .by_account_id(&account.id)?
        .first()
        .cloned()
        .ok_or(NotFound)?;
    let eventable = db.eventables().create(&Eventable::Lease(lease.clone()))?;

    notify(
        db,
        event.clone().into(),
        eventable.id(),
        account.id,
        sender.id,
        participant.id,
        None,
    )?;

    Ok(())
}

fn on_payment_created(db: &Pg, event: &Event, payment: &Payment) -> Result<()> {
    let account = db.accounts().by_payment_id(&payment.id)?;
    let participant = db.persons().by_payment_id(&payment.id)?;
    let eventable = db
        .eventables()
        .create(&Eventable::Payment(payment.clone()))?;

    notify(
        db,
        event.clone().into(),
        eventable.id(),
        account.id,
        participant.id,
        participant.id,
        None,
    )?;

    Ok(())
}

fn on_notice_created(db: &Pg, event: &Event, notice: &Notice) -> Result<()> {
    let account = db.accounts().by_notice_id(&notice.id)?;
    let participant = db.persons().by_notice_id(&notice.id)?;
    let eventable = db.eventables().create(&Eventable::File(notice.clone()))?;

    notify(
        db,
        event.clone().into(),
        eventable.id(),
        account.id,
        participant.id,
        participant.id,
        None,
    )?;

    Ok(())
}

fn on_receipt_created(db: &Pg, event: &Event, receipt: &Receipt) -> Result<()> {
    let account = db.accounts().by_receipt_id(&receipt.id)?;
    let participant = db.persons().by_receipt_id(&receipt.id)?;
    let eventable = db.eventables().create(&Eventable::File(receipt.clone()))?;

    notify(
        db,
        event.clone().into(),
        eventable.id(),
        account.id,
        participant.id,
        participant.id,
        None,
    )?;

    Ok(())
}

fn on_step_completed(db: &Pg, event: &Event, step: &Step) -> Result<()> {
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
        }
    }

    let message = render_step_message(db, step.clone(), participant.clone())?;

    notify(
        db,
        event.clone().into(),
        eventable.id(),
        account.id,
        sender.id,
        participant.id,
        message,
    )?;

    Ok(())
}

// # Utils

fn notify(
    db: &Pg,
    type_: EventType,
    eventable_id: EventableId,
    account_id: AccountId,
    sender_id: PersonId,
    participant_id: PersonId,
    content: Option<String>,
) -> Result<Message> {
    let event = db.events().create(&trankeel_data::Event {
        id: EventId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        account_id,
        participant_id,
        eventable_id,
        type_,
    })?;

    let discussion = db.discussions().by_initiator_id(&participant_id)?;

    db.messages().create(&Message {
        id: MessageId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        discussion_id: discussion.id,
        sender_id,
        content,
        event_id: Some(event.id),
    })
}

fn render_step_message(db: &Pg, step: Step, participant: Person) -> Result<Option<String>> {
    let tenant = db.tenants().by_person_id(&participant.id)?;
    let lease = db.leases().by_tenant_id(&tenant.id)?; // TODO: match workflowable
    Ok(step.confirmation.map(|message| {
        message
            .replace("{{ tenant_name }}", &participant.display_name())
            .replace("{{ effect_date }}", &lease.effect_date.inner().to_rfc3339())
            .replace(
                "{{ deposit_amount }}",
                &format!(
                    "{} {}",
                    lease.deposit_amount.inner().round_dp(2),
                    DEFAULT_CURRENCY
                ),
            )
    }))
}
