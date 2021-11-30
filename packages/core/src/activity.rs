use crate::context::Context;
use crate::error::Result;
use crate::handlers::candidacy_accepted::candidacy_accepted;
use crate::handlers::candidacy_created::candidacy_created;
use crate::handlers::lease_created::lease_created;
use crate::handlers::notice_created::notice_created;
use crate::handlers::payment_created::payment_created;
use crate::handlers::receipt_created::receipt_created;
use crate::handlers::step_completed::step_completed;
use crate::providers::Pg;
use trankeel_data::Candidacy;
use trankeel_data::EventType;
use trankeel_data::File;
use trankeel_data::Lease;
use trankeel_data::Payment;
use trankeel_data::Step;

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
    let ctx = Context::env();

    Pg::transaction(ctx.db(), || {
        events.iter().try_for_each(|event| match event {
            Event::CandidacyCreated(candidacy) => candidacy_created(&ctx, event, candidacy),
            Event::CandidacyAccepted(candidacy) => candidacy_accepted(&ctx, event, candidacy),
            Event::CandidacyRejected(candidacy) => candidacy_accepted(&ctx, event, candidacy),
            Event::LeaseCreated(lease) => lease_created(&ctx, event, lease),
            Event::PaymentCreated(payment) => payment_created(&ctx, event, payment),
            Event::NoticeCreated(notice) => notice_created(&ctx, event, notice),
            Event::NoticeSent(notice) => notice_created(&ctx, event, notice),
            Event::ReceiptCreated(receipt) => receipt_created(&ctx, event, receipt),
            Event::ReceiptSent(receipt) => receipt_created(&ctx, event, receipt),
            Event::StepCompleted(step) => step_completed(&ctx, event, step),
        })
    })
}
