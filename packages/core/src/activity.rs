use crate::database::Db;
use crate::error::Result;
use diesel::result::Error::NotFound;
use trankeel_data::AccountId;
use trankeel_data::Candidacy;
use trankeel_data::EventId;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::EventableId;
use trankeel_data::File;
use trankeel_data::Message;
use trankeel_data::MessageId;
use trankeel_data::Notice;
use trankeel_data::Payment;
use trankeel_data::PersonId;
use trankeel_data::Receipt;

type Meta = (EventableId, AccountId, PersonId, PersonId);

#[derive(Clone)]
pub enum Trace {
    CandidacyCreated(Candidacy),
    CandidacyAccepted(Candidacy),
    CandidacyRejected(Candidacy),
    PaymentCreated(Payment),
    NoticeCreated(File),
    NoticeSent(File),
    ReceiptCreated(File),
    ReceiptSent(File),
}

impl From<Trace> for EventType {
    fn from(item: Trace) -> Self {
        match item {
            Trace::CandidacyCreated(_) => Self::CandidacyCreated,
            Trace::CandidacyAccepted(_) => Self::CandidacyAccepted,
            Trace::CandidacyRejected(_) => Self::CandidacyRejected,
            Trace::PaymentCreated(_) => Self::PaymentCreated,
            Trace::NoticeCreated(_) => Self::NoticeCreated,
            Trace::NoticeSent(_) => Self::NoticeSent,
            Trace::ReceiptCreated(_) => Self::ReceiptCreated,
            Trace::ReceiptSent(_) => Self::ReceiptSent,
        }
    }
}

pub fn trace(db: &impl Db, trace: Trace) -> Result<Trace> {
    let (eventable_id, account_id, sender_id, participant_id) = match &trace {
        Trace::CandidacyCreated(candidacy) => on_candidacy_created(db, candidacy.clone())?,
        Trace::CandidacyAccepted(candidacy) => on_candidacy_accepted(db, candidacy.clone())?,
        Trace::CandidacyRejected(candidacy) => on_candidacy_accepted(db, candidacy.clone())?,
        Trace::PaymentCreated(payment) => on_payment_created(db, payment.clone())?,
        Trace::NoticeCreated(notice) => on_notice_created(db, notice.clone())?,
        Trace::NoticeSent(notice) => on_notice_created(db, notice.clone())?,
        Trace::ReceiptCreated(receipt) => on_receipt_created(db, receipt.clone())?,
        Trace::ReceiptSent(receipt) => on_receipt_created(db, receipt.clone())?,
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
        content: None,
        event_id: Some(event.id),
    })?;

    Ok(trace)
}

fn on_candidacy_created(db: &impl Db, candidacy: Candidacy) -> Result<Meta> {
    let account = db.accounts().by_candidacy_id(&candidacy.id)?;
    let participant = db.persons().by_candidacy_id(&candidacy.id)?;
    let eventable = db.eventables().create(Eventable::Candidacy(candidacy))?;

    Ok((eventable.id(), account.id, participant.id, participant.id))
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

    Ok((eventable.id(), account.id, sender.id, participant.id))
}

fn on_payment_created(db: &impl Db, payment: Payment) -> Result<Meta> {
    let account = db.accounts().by_payment_id(&payment.id)?;
    let participant = db.persons().by_payment_id(&payment.id)?;
    let eventable = db.eventables().create(Eventable::Payment(payment))?;

    Ok((eventable.id(), account.id, participant.id, participant.id))
}

fn on_notice_created(db: &impl Db, notice: Notice) -> Result<Meta> {
    let account = db.accounts().by_notice_id(&notice.id)?;
    let participant = db.persons().by_notice_id(&notice.id)?;
    let eventable = db.eventables().create(Eventable::File(notice))?;

    Ok((eventable.id(), account.id, participant.id, participant.id))
}

fn on_receipt_created(db: &impl Db, receipt: Receipt) -> Result<Meta> {
    let account = db.accounts().by_receipt_id(&receipt.id)?;
    let participant = db.persons().by_receipt_id(&receipt.id)?;
    let eventable = db.eventables().create(Eventable::File(receipt))?;

    Ok((eventable.id(), account.id, participant.id, participant.id))
}
