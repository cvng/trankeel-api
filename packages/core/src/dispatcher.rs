use crate::context::Context;
use crate::error::Result;
use crate::handlers::advertisement_created;
use crate::handlers::advertisement_updated;
use crate::handlers::candidacy_accepted;
use crate::handlers::candidacy_created;
use crate::handlers::candidacy_rejected;
use crate::handlers::lease_created;
use crate::handlers::notice_created;
use crate::handlers::notice_sent;
use crate::handlers::payment_created;
use crate::handlers::property_created;
use crate::handlers::receipt_created;
use crate::handlers::receipt_sent;
use crate::handlers::step_completed;
use crate::handlers::tenant_created;
use crate::handlers::AdvertisementCreated;
use crate::handlers::AdvertisementUpdated;
use crate::handlers::CandidacyRejected;
use crate::handlers::LeaseCreated;
use crate::handlers::PropertyCreated;
use crate::handlers::TenantCreated;
use crate::providers::Pg;
use trankeel_data::Candidacy;
use trankeel_data::EventType;
use trankeel_data::File;
use trankeel_data::Payment;
use trankeel_data::Step;

#[async_trait]
pub trait AsyncCommand {
    type Input;
    type Payload;

    async fn run(&self, input: Self::Input) -> Result<Self::Payload>;
}

pub trait Command {
    type Input;

    fn run(self, input: Self::Input) -> Result<Vec<Event>>;
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum Event {
    AdvertisementCreated(AdvertisementCreated),
    AdvertisementUpdated(AdvertisementUpdated),
    CandidacyAccepted(Candidacy),
    CandidacyCreated(Candidacy),
    CandidacyRejected(CandidacyRejected),
    LeaseCreated(LeaseCreated),
    NoticeCreated(File),
    NoticeSent(File),
    PaymentCreated(Payment),
    PropertyCreated(PropertyCreated),
    ReceiptCreated(File),
    ReceiptSent(File),
    StepCompleted(Step),
    TenantCreated(TenantCreated),
}

impl From<Event> for EventType {
    fn from(item: Event) -> Self {
        match item {
            Event::AdvertisementCreated(_) => unimplemented!(),
            Event::AdvertisementUpdated(_) => unimplemented!(),
            Event::CandidacyAccepted(_) => Self::CandidacyAccepted,
            Event::CandidacyCreated(_) => Self::CandidacyCreated,
            Event::CandidacyRejected(_) => Self::CandidacyRejected,
            Event::LeaseCreated(_) => Self::LeaseCreated,
            Event::NoticeCreated(_) => Self::NoticeCreated,
            Event::NoticeSent(_) => Self::NoticeSent,
            Event::PaymentCreated(_) => Self::PaymentCreated,
            Event::PropertyCreated(_) => unimplemented!(),
            Event::ReceiptCreated(_) => Self::ReceiptCreated,
            Event::ReceiptSent(_) => Self::ReceiptSent,
            Event::StepCompleted(_) => Self::StepCompleted,
            Event::TenantCreated(_) => unimplemented!(),
        }
    }
}

pub fn dispatch(events: Vec<Event>) -> Result<Vec<Event>> {
    let ctx = Context::env();

    Pg::transaction(ctx.db(), || {
        events.iter().try_for_each(|event| match event {
            Event::AdvertisementCreated(event) => advertisement_created(&ctx, event.clone()),
            Event::AdvertisementUpdated(event) => advertisement_updated(&ctx, event.clone()),
            Event::CandidacyAccepted(candidacy) => candidacy_accepted(&ctx, event, candidacy),
            Event::CandidacyCreated(candidacy) => candidacy_created(&ctx, event, candidacy),
            Event::CandidacyRejected(event) => candidacy_rejected(&ctx, event.clone()),
            Event::LeaseCreated(event) => lease_created(&ctx, event.clone()),
            Event::NoticeCreated(notice) => notice_created(&ctx, event, notice),
            Event::NoticeSent(notice) => notice_sent(&ctx, event, notice),
            Event::PaymentCreated(payment) => payment_created(&ctx, event, payment),
            Event::PropertyCreated(event) => property_created(&ctx, event.clone()),
            Event::ReceiptCreated(receipt) => receipt_created(&ctx, event, receipt),
            Event::ReceiptSent(receipt) => receipt_sent(&ctx, event, receipt),
            Event::StepCompleted(step) => step_completed(&ctx, event, step),
            Event::TenantCreated(event) => tenant_created(&ctx, event.clone()),
        })
    })?;

    Ok(events)
}
