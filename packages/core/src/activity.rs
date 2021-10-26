use crate::database::Db;
use crate::error::Result;
use chrono::DateTime;
use chrono::Utc;
use diesel::result::Error::NotFound;
use trankeel_data::AccountId;
use trankeel_data::Candidacy;
use trankeel_data::EventId;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::EventableId;
use trankeel_data::File;
use trankeel_data::Lease;
use trankeel_data::LeaseData;
use trankeel_data::Message;
use trankeel_data::MessageId;
use trankeel_data::Name;
use trankeel_data::Notice;
use trankeel_data::Payment;
use trankeel_data::Person;
use trankeel_data::PersonId;
use trankeel_data::Receipt;
use trankeel_data::Step;
use trankeel_kit::locale::DEFAULT_CURRENCY;

type Meta = (EventableId, AccountId, PersonId, PersonId, Option<String>);

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum Trace {
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

impl From<Trace> for EventType {
    fn from(item: Trace) -> Self {
        match item {
            Trace::CandidacyCreated(_) => Self::CandidacyCreated,
            Trace::CandidacyAccepted(_) => Self::CandidacyAccepted,
            Trace::CandidacyRejected(_) => Self::CandidacyRejected,
            Trace::LeaseCreated(_) => Self::LeaseCreated,
            Trace::PaymentCreated(_) => Self::PaymentCreated,
            Trace::NoticeCreated(_) => Self::NoticeCreated,
            Trace::NoticeSent(_) => Self::NoticeSent,
            Trace::ReceiptCreated(_) => Self::ReceiptCreated,
            Trace::ReceiptSent(_) => Self::ReceiptSent,
            Trace::StepCompleted(_) => Self::StepCompleted,
        }
    }
}

pub fn trace(db: &impl Db, trace: Trace) -> Result<Trace> {
    let (eventable_id, account_id, sender_id, participant_id, content) = match &trace {
        Trace::CandidacyCreated(candidacy) => on_candidacy_created(db, candidacy.clone())?,
        Trace::CandidacyAccepted(candidacy) => on_candidacy_accepted(db, candidacy.clone())?,
        Trace::CandidacyRejected(candidacy) => on_candidacy_accepted(db, candidacy.clone())?,
        Trace::LeaseCreated(lease) => on_lease_created(db, lease.clone())?,
        Trace::PaymentCreated(payment) => on_payment_created(db, payment.clone())?,
        Trace::NoticeCreated(notice) => on_notice_created(db, notice.clone())?,
        Trace::NoticeSent(notice) => on_notice_created(db, notice.clone())?,
        Trace::ReceiptCreated(receipt) => on_receipt_created(db, receipt.clone())?,
        Trace::ReceiptSent(receipt) => on_receipt_created(db, receipt.clone())?,
        Trace::StepCompleted(step) => on_step_completed(db, step.clone())?,
    };

    let event = db.events().create(trankeel_data::Event {
        id: EventId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        account_id,
        participant_id,
        eventable_id,
        type_: trace.clone().into(),
    })?;

    let discussion = db.discussions().by_initiator_id(&participant_id)?;

    db.messages().create(Message {
        id: MessageId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        discussion_id: discussion.id,
        sender_id,
        content,
        event_id: Some(event.id),
    })?;

    Ok(trace)
}

fn on_candidacy_created(db: &impl Db, candidacy: Candidacy) -> Result<Meta> {
    let account = db.accounts().by_candidacy_id(&candidacy.id)?;
    let participant = db.persons().by_candidacy_id(&candidacy.id)?;
    let eventable = db.eventables().create(Eventable::Candidacy(candidacy))?;

    Ok((
        eventable.id(),
        account.id,
        participant.id,
        participant.id,
        None,
    ))
}

fn on_candidacy_accepted(db: &impl Db, candidacy: Candidacy) -> Result<Meta> {
    let account = db.accounts().by_candidacy_id(&candidacy.id)?;
    let participant = db.persons().by_candidacy_id(&candidacy.id)?;
    let sender = db
        .persons()
        .by_account_id(&account.id)?
        .first()
        .cloned()
        .ok_or(NotFound)?;
    let eventable = db.eventables().create(Eventable::Candidacy(candidacy))?;

    Ok((eventable.id(), account.id, sender.id, participant.id, None))
}

fn on_lease_created(db: &impl Db, lease: Lease) -> Result<Meta> {
    let account = db.accounts().by_lease_id(&lease.id)?;
    let participant = db.persons().by_lease_id(&lease.id)?;
    let sender = db
        .persons()
        .by_account_id(&account.id)?
        .first()
        .cloned()
        .ok_or(NotFound)?;
    let eventable = db.eventables().create(Eventable::Lease(lease))?;

    Ok((eventable.id(), account.id, sender.id, participant.id, None))
}

fn on_payment_created(db: &impl Db, payment: Payment) -> Result<Meta> {
    let account = db.accounts().by_payment_id(&payment.id)?;
    let participant = db.persons().by_payment_id(&payment.id)?;
    let eventable = db.eventables().create(Eventable::Payment(payment))?;

    Ok((
        eventable.id(),
        account.id,
        participant.id,
        participant.id,
        None,
    ))
}

fn on_notice_created(db: &impl Db, notice: Notice) -> Result<Meta> {
    let account = db.accounts().by_notice_id(&notice.id)?;
    let participant = db.persons().by_notice_id(&notice.id)?;
    let eventable = db.eventables().create(Eventable::File(notice))?;

    Ok((
        eventable.id(),
        account.id,
        participant.id,
        participant.id,
        None,
    ))
}

fn on_receipt_created(db: &impl Db, receipt: Receipt) -> Result<Meta> {
    let account = db.accounts().by_receipt_id(&receipt.id)?;
    let participant = db.persons().by_receipt_id(&receipt.id)?;
    let eventable = db.eventables().create(Eventable::File(receipt))?;

    Ok((
        eventable.id(),
        account.id,
        participant.id,
        participant.id,
        None,
    ))
}

fn on_step_completed(db: &impl Db, step: Step) -> Result<Meta> {
    let account = db.accounts().by_step_id(&step.id)?;
    let participant = db.persons().by_step_id(&step.id)?;
    let sender = db
        .persons()
        .by_account_id(&account.id)?
        .first()
        .cloned()
        .ok_or(NotFound)?;
    let eventable = db.eventables().create(Eventable::Step(step.clone()))?;
    let message = render_step_message(db, step.clone(), participant.clone())?;

    if step.label == LEASE_SIGNED_EVENT_LABEL {
        let lease = db.leases().by_person_id(&participant.id)?;
        db.leases().update(LeaseData {
            id: lease.id,
            signature_date: Some(Utc::now().into()), // TODO: match workflowable
            ..Default::default()
        })?;
    }

    if step.label == LEASE_STARTED_EVENT_LABEL {
        let step_requirements = match step.requirements {
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

    Ok((
        eventable.id(),
        account.id,
        sender.id,
        participant.id,
        message,
    ))
}

// # Utils

const LEASE_SIGNED_EVENT_LABEL: &str = "Signature du contrat de location";

const LEASE_STARTED_EVENT_LABEL: &str = "Confirmation de la date de remise des clés";

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
