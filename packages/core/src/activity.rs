use crate::database::Db;
use crate::error::Result;
use piteo_data::AccountId;
use piteo_data::Candidacy;
use piteo_data::EventId;
use piteo_data::EventType;
use piteo_data::EventableId;
use piteo_data::File;
use piteo_data::Message;
use piteo_data::MessageId;
use piteo_data::Notice;
use piteo_data::Payment;
use piteo_data::PersonId;
use piteo_data::Receipt;

type Meta = (EventableId, AccountId, PersonId);

#[derive(Clone)]
pub enum Trace {
    CandidacyCreated(Candidacy),
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
            Trace::PaymentCreated(_) => Self::PaymentCreated,
            Trace::NoticeCreated(_) => Self::NoticeCreated,
            Trace::NoticeSent(_) => Self::NoticeSent,
            Trace::ReceiptCreated(_) => Self::ReceiptCreated,
            Trace::ReceiptSent(_) => Self::ReceiptSent,
        }
    }
}

pub fn trace(db: &impl Db, trace: Trace) -> Result<Trace> {
    let (eventable_id, account_id, participant_id) = match &trace {
        Trace::CandidacyCreated(candidacy) => on_candidacy_created(db, candidacy)?,
        Trace::PaymentCreated(payment) => on_payment_created(db, payment)?,
        Trace::NoticeCreated(file) => on_notice_created(db, file)?,
        Trace::NoticeSent(file) => on_notice_created(db, file)?,
        Trace::ReceiptCreated(file) => on_receipt_created(db, file)?,
        Trace::ReceiptSent(file) => on_receipt_created(db, file)?,
    };

    let event = db.events().create(piteo_data::Event {
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
        sender_id: participant_id,
        content: None,
        event_id: Some(event.id),
    })?;

    Ok(trace)
}

fn on_candidacy_created(db: &impl Db, eventable: &Candidacy) -> Result<Meta> {
    let account = db.accounts().by_candidacy_id(&eventable.id)?;
    let participant = db.persons().by_candidacy_id(&eventable.id)?;

    Ok((eventable.id, account.id, participant.id))
}

fn on_payment_created(db: &impl Db, eventable: &Payment) -> Result<Meta> {
    let account = db.accounts().by_payment_id(&eventable.id)?;
    let participant = db.persons().by_payment_id(&eventable.id)?;

    Ok((eventable.id, account.id, participant.id))
}

fn on_notice_created(db: &impl Db, eventable: &Notice) -> Result<Meta> {
    let account = db.accounts().by_notice_id(&eventable.id)?;
    let participant = db.persons().by_notice_id(&eventable.id)?;

    Ok((eventable.id, account.id, participant.id))
}

fn on_receipt_created(db: &impl Db, eventable: &Receipt) -> Result<Meta> {
    let account = db.accounts().by_receipt_id(&eventable.id)?;
    let participant = db.persons().by_receipt_id(&eventable.id)?;

    Ok((eventable.id, account.id, participant.id))
}
