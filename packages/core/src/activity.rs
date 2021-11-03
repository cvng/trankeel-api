use crate::database::Db;
use crate::error::Result;
use crate::providers::Pg;
use chrono::DateTime;
use chrono::Utc;
use diesel::result::Error::NotFound;
use serde_json::to_value;
use trankeel_data::Candidacy;
use trankeel_data::Discussion;
use trankeel_data::DiscussionWithMessages;
use trankeel_data::Event;
use trankeel_data::EventId;
use trankeel_data::EventPayload;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::EventableType;
use trankeel_data::File;
use trankeel_data::Lease;
use trankeel_data::LeaseData;
use trankeel_data::Message;
use trankeel_data::MessageId;
use trankeel_data::Name;
use trankeel_data::Payment;
use trankeel_data::Person;
use trankeel_data::PersonId;
use trankeel_data::PublicEvent;
use trankeel_data::PublicEventEventable;
use trankeel_data::PublicEventId;
use trankeel_data::Step;
use trankeel_data::TenantWithProfile;
use trankeel_data::WarrantWithIdentity;
use trankeel_kit::locale::DEFAULT_CURRENCY;

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum Trace {
    CandidacyCreated(Candidacy),
    CandidacyAccepted(Candidacy),
    CandidacyRejected(Candidacy),
    DiscussionCreated(DiscussionWithMessages),
    DiscussionUpdated(Discussion),
    LeaseCreated(Lease),
    MessageCreated(Message),
    NoticeCreated(File),
    NoticeSent(File),
    PaymentCreated(Payment),
    ReceiptCreated(File),
    ReceiptSent(File),
    StepCompleted(Step),
    TenantCreated(TenantWithProfile),
    WarrantCreated(WarrantWithIdentity),
}

impl From<Trace> for Eventable {
    fn from(item: Trace) -> Self {
        match item {
            Trace::CandidacyCreated(inner) => Self::Candidacy(inner),
            Trace::CandidacyAccepted(inner) => Self::Candidacy(inner),
            Trace::CandidacyRejected(inner) => Self::Candidacy(inner),
            Trace::DiscussionCreated(inner) => Self::Discussion(inner.0),
            Trace::DiscussionUpdated(inner) => Self::Discussion(inner),
            Trace::LeaseCreated(inner) => Self::Lease(inner),
            Trace::MessageCreated(inner) => Self::Message(inner),
            Trace::NoticeCreated(inner) => Self::File(inner),
            Trace::NoticeSent(inner) => Self::File(inner),
            Trace::PaymentCreated(inner) => Self::Payment(inner),
            Trace::ReceiptCreated(inner) => Self::File(inner),
            Trace::ReceiptSent(inner) => Self::File(inner),
            Trace::StepCompleted(inner) => Self::Step(inner),
            Trace::TenantCreated(inner) => Self::Tenant(inner.0),
            Trace::WarrantCreated(inner) => Self::Warrant(inner.0),
        }
    }
}

impl From<Trace> for EventType {
    fn from(item: Trace) -> Self {
        match item {
            Trace::CandidacyCreated(_) => Self::CandidacyCreated,
            Trace::CandidacyAccepted(_) => Self::CandidacyAccepted,
            Trace::CandidacyRejected(_) => Self::CandidacyRejected,
            Trace::DiscussionCreated(_) => Self::DiscussionCreated,
            Trace::DiscussionUpdated(_) => Self::DiscussionUpdated,
            Trace::LeaseCreated(_) => Self::LeaseCreated,
            Trace::MessageCreated(_) => Self::MessageCreated,
            Trace::PaymentCreated(_) => Self::PaymentCreated,
            Trace::NoticeCreated(_) => Self::NoticeCreated,
            Trace::NoticeSent(_) => Self::NoticeSent,
            Trace::ReceiptCreated(_) => Self::ReceiptCreated,
            Trace::ReceiptSent(_) => Self::ReceiptSent,
            Trace::StepCompleted(_) => Self::StepCompleted,
            Trace::TenantCreated(_) => Self::TenantCreated,
            Trace::WarrantCreated(_) => Self::WarrantCreated,
        }
    }
}

impl From<Trace> for EventPayload {
    fn from(item: Trace) -> Self {
        match item {
            Trace::CandidacyCreated(inner) => to_value(inner).unwrap().into(),
            Trace::CandidacyAccepted(inner) => to_value(inner).unwrap().into(),
            Trace::CandidacyRejected(inner) => to_value(inner).unwrap().into(),
            Trace::DiscussionCreated(inner) => to_value(inner).unwrap().into(),
            Trace::DiscussionUpdated(inner) => to_value(inner).unwrap().into(),
            Trace::LeaseCreated(inner) => to_value(inner).unwrap().into(),
            Trace::MessageCreated(inner) => to_value(inner).unwrap().into(),
            Trace::NoticeCreated(inner) => to_value(inner).unwrap().into(),
            Trace::NoticeSent(inner) => to_value(inner).unwrap().into(),
            Trace::PaymentCreated(inner) => to_value(inner).unwrap().into(),
            Trace::ReceiptCreated(inner) => to_value(inner).unwrap().into(),
            Trace::ReceiptSent(inner) => to_value(inner).unwrap().into(),
            Trace::StepCompleted(inner) => to_value(inner).unwrap().into(),
            Trace::TenantCreated(inner) => to_value(inner).unwrap().into(),
            Trace::WarrantCreated(inner) => to_value(inner).unwrap().into(),
        }
    }
}

impl From<Trace> for PublicEventEventable {
    fn from(item: Trace) -> Self {
        match item {
            Trace::CandidacyCreated(inner) => to_value(inner).unwrap(),
            Trace::CandidacyAccepted(inner) => to_value(inner).unwrap(),
            Trace::CandidacyRejected(inner) => to_value(inner).unwrap(),
            Trace::DiscussionCreated(inner) => to_value(inner.0).unwrap(),
            Trace::DiscussionUpdated(inner) => to_value(inner).unwrap(),
            Trace::LeaseCreated(inner) => to_value(inner).unwrap(),
            Trace::MessageCreated(inner) => to_value(inner).unwrap(),
            Trace::NoticeCreated(inner) => to_value(inner).unwrap(),
            Trace::NoticeSent(inner) => to_value(inner).unwrap(),
            Trace::PaymentCreated(inner) => to_value(inner).unwrap(),
            Trace::ReceiptCreated(inner) => to_value(inner).unwrap(),
            Trace::ReceiptSent(inner) => to_value(inner).unwrap(),
            Trace::StepCompleted(inner) => to_value(inner).unwrap(),
            Trace::TenantCreated(inner) => to_value(inner.0).unwrap(),
            Trace::WarrantCreated(inner) => to_value(inner.0).unwrap(),
        }
    }
}

impl From<Trace> for EventableType {
    fn from(item: Trace) -> Self {
        match item {
            Trace::CandidacyCreated(_) => Self::Candidacy,
            Trace::CandidacyAccepted(_) => Self::Candidacy,
            Trace::CandidacyRejected(_) => Self::Candidacy,
            Trace::DiscussionCreated(_) => Self::Discussion,
            Trace::DiscussionUpdated(_) => Self::Discussion,
            Trace::LeaseCreated(_) => Self::Lease,
            Trace::MessageCreated(_) => Self::Message,
            Trace::NoticeCreated(_) => Self::File,
            Trace::NoticeSent(_) => Self::File,
            Trace::PaymentCreated(_) => Self::Payment,
            Trace::ReceiptCreated(_) => Self::File,
            Trace::ReceiptSent(_) => Self::File,
            Trace::StepCompleted(_) => Self::Step,
            Trace::TenantCreated(_) => Self::Tenant,
            Trace::WarrantCreated(_) => Self::Warrant,
        }
    }
}

pub fn trace(events: Vec<Trace>) -> Result<()> {
    let db = &Pg::init();

    events.into_iter().into_iter().try_for_each(|trace| {
        let event = persist(db, trace.clone())?;

        on_candidacy_created(db, event.clone(), trace.clone())?;
        on_candidacy_accepted(db, event.clone(), trace.clone())?;
        on_discussion_created(db, event.clone(), trace.clone())?;
        on_discussion_updated(db, event.clone(), trace.clone())?;
        on_lease_created(db, event.clone(), trace.clone())?;
        on_message_created(db, event.clone(), trace.clone())?;
        on_notice_created(db, event.clone(), trace.clone())?;
        on_payment_created(db, event.clone(), trace.clone())?;
        on_receipt_created(db, event.clone(), trace.clone())?;
        on_step_completed(db, event.clone(), trace.clone())?;
        on_tenant_created(db, event.clone(), trace.clone())?;
        on_warrant_created(db, event.clone(), trace.clone())?;

        on_event_created(db, event, trace)?;

        Ok(())
    })
}

fn persist(db: &impl Db, trace: Trace) -> Result<Event> {
    db.events().create(Event {
        id: EventId::new(),
        created_at: Default::default(),
        type_: trace.clone().into(),
        payload: trace.into(),
    })
}

// # Handlers

fn on_event_created(db: &impl Db, event: Event, trace: Trace) -> Result<()> {
    let eventable_type: EventableType = trace.clone().into();
    let eventable: PublicEventEventable = trace.clone().into();
    let account = match trace.into() {
        Eventable::Advertisement(inner) => db.accounts().by_advertisement_id(&inner.id)?,
        Eventable::Candidacy(inner) => db.accounts().by_candidacy_id(&inner.id)?,
        Eventable::Discussion(inner) => db.accounts().by_discussion_id(&inner.id)?,
        Eventable::File(inner) => db.accounts().by_file_id(&inner.id)?,
        Eventable::Lease(inner) => db.accounts().by_lease_id(&inner.id)?,
        Eventable::Message(inner) => db.accounts().by_message_id(&inner.id)?,
        Eventable::Payment(inner) => db.accounts().by_payment_id(&inner.id)?,
        Eventable::Person(inner) => db.accounts().by_person_id(&inner.id)?,
        Eventable::Rent(inner) => db.accounts().by_rent_id(&inner.id)?,
        Eventable::Step(inner) => db.accounts().by_step_id(&inner.id)?,
        Eventable::Tenant(inner) => db.accounts().by_tenant_id(&inner.id)?,
        Eventable::Warrant(inner) => db.accounts().by_warrant_id(&inner.id)?,
    };
    db.public_events().create(PublicEvent {
        id: PublicEventId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        account_id: account.id,
        event_id: event.id,
        event_type: event.type_,
        eventable_type,
        eventable,
    })?;
    Ok(())
}

fn on_candidacy_created(db: &impl Db, event: Event, trace: Trace) -> Result<()> {
    match trace {
        Trace::CandidacyCreated(ref candidacy) => {
            let participant = db.persons().by_candidacy_id(&candidacy.id)?;
            push_message(db, event, participant.id, participant.id, None)?;
            Ok(())
        }
        _ => Ok(()),
    }
}

fn on_candidacy_accepted(db: &impl Db, event: Event, trace: Trace) -> Result<()> {
    match trace {
        Trace::CandidacyAccepted(ref candidacy) => {
            let account = db.accounts().by_candidacy_id(&candidacy.id)?;
            let participant = db.persons().by_candidacy_id(&candidacy.id)?;
            let sender = db
                .persons()
                .by_account_id(&account.id)?
                .first()
                .cloned()
                .ok_or(NotFound)?;
            push_message(db, event, sender.id, participant.id, None)?;
            Ok(())
        }
        _ => Ok(()),
    }
}

fn on_discussion_created(db: &impl Db, _event: Event, trace: Trace) -> Result<()> {
    match trace {
        Trace::DiscussionCreated((ref discussion, ref messages)) => {
            db.discussions().create(discussion.clone())?;
            db.messages().create_many(messages.clone())?;
            Ok(())
        }
        _ => Ok(()),
    }
}

fn on_discussion_updated(db: &impl Db, _event: Event, trace: Trace) -> Result<()> {
    match trace {
        Trace::DiscussionUpdated(ref discussion) => {
            db.discussions().update(discussion.clone().into())?;
            Ok(())
        }
        _ => Ok(()),
    }
}

fn on_lease_created(db: &impl Db, event: Event, trace: Trace) -> Result<()> {
    match trace {
        Trace::LeaseCreated(ref lease) => {
            let account = db.accounts().by_lease_id(&lease.id)?;
            let participant = db.persons().by_lease_id(&lease.id)?;
            let sender = db
                .persons()
                .by_account_id(&account.id)?
                .first()
                .cloned()
                .ok_or(NotFound)?;
            push_message(db, event, sender.id, participant.id, None)?;
            Ok(())
        }
        _ => Ok(()),
    }
}

fn on_message_created(db: &impl Db, _event: Event, trace: Trace) -> Result<()> {
    match trace {
        Trace::MessageCreated(ref message) => {
            db.messages().create_many(vec![message.clone()])?;
            Ok(())
        }
        _ => Ok(()),
    }
}

fn on_notice_created(db: &impl Db, event: Event, trace: Trace) -> Result<()> {
    match trace {
        Trace::NoticeCreated(ref notice) => {
            let participant = db.persons().by_notice_id(&notice.id)?;
            push_message(db, event, participant.id, participant.id, None)?;
            Ok(())
        }
        _ => Ok(()),
    }
}

fn on_payment_created(db: &impl Db, event: Event, trace: Trace) -> Result<()> {
    match trace {
        Trace::PaymentCreated(ref payment) => {
            let participant = db.persons().by_payment_id(&payment.id)?;
            push_message(db, event, participant.id, participant.id, None)?;
            Ok(())
        }
        _ => Ok(()),
    }
}

fn on_receipt_created(db: &impl Db, event: Event, trace: Trace) -> Result<()> {
    match trace {
        Trace::ReceiptCreated(ref receipt) => {
            let participant = db.persons().by_receipt_id(&receipt.id)?;
            push_message(db, event, participant.id, participant.id, None)?;
            Ok(())
        }
        _ => Ok(()),
    }
}

fn on_step_completed(db: &impl Db, event: Event, trace: Trace) -> Result<()> {
    match trace {
        Trace::StepCompleted(ref step) => {
            const LEASE_SIGNED_EVENT_LABEL: &str = "Signature du contrat de location";
            const LEASE_STARTED_EVENT_LABEL: &str = "Confirmation de la date de remise des clés";

            let account = db.accounts().by_step_id(&step.id)?;
            let participant = db.persons().by_step_id(&step.id)?;
            let sender = db
                .persons()
                .by_account_id(&account.id)?
                .first()
                .cloned()
                .ok_or(NotFound)?;
            if step.label == LEASE_SIGNED_EVENT_LABEL {
                let lease = db.leases().by_person_id(&participant.id)?;
                db.leases().update(LeaseData {
                    id: lease.id,
                    signature_date: Some(Utc::now().into()), // TODO: match workflowable
                    ..Default::default()
                })?;
            }
            if step.label == LEASE_STARTED_EVENT_LABEL {
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
                    db.leases().update(LeaseData {
                        id: lease.id,
                        effect_date: Some(effect_date.parse::<DateTime<Utc>>()?.into()), // TODO: match workflowable
                        ..Default::default()
                    })?;
                }
            }
            let message = render_step_message(db, step.clone(), participant.clone())?;
            push_message(db, event, sender.id, participant.id, message)?;
            Ok(())
        }
        _ => Ok(()),
    }
}

fn on_tenant_created(db: &impl Db, _event: Event, trace: Trace) -> Result<()> {
    match trace {
        Trace::TenantCreated((ref tenant, ref person)) => {
            db.persons().create(person.clone())?;
            db.tenants().create(tenant.clone())?;
            Ok(())
        }
        _ => Ok(()),
    }
}

fn on_warrant_created(db: &impl Db, _event: Event, trace: Trace) -> Result<()> {
    match trace {
        Trace::WarrantCreated(ref warrant) => {
            db.warrants().create(warrant.clone())?;
            Ok(())
        }
        _ => Ok(()),
    }
}

// # Utils

fn push_message(
    db: &impl Db,
    event: Event,
    sender_id: PersonId,
    participant_id: PersonId,
    content: Option<String>,
) -> Result<Message> {
    let discussion = db.discussions().by_initiator_id(&participant_id)?;

    db.messages().create(Message {
        id: MessageId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        discussion_id: discussion.id,
        sender_id,
        content,
        event_id: Some(event.id),
    })
}

fn render_step_message(db: &impl Db, step: Step, participant: Person) -> Result<Option<String>> {
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
