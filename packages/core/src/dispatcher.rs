use crate::context::Context;
use crate::error::Result;
use crate::handlers::*;
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
    LeaseAffected(LeaseAffected),
    LeaseCreated(LeaseCreated),
    NoticeCreated(File),
    PaymentCreated(Payment),
    PropertyCreated(PropertyCreated),
    PropertyUpdated(PropertyUpdated),
    ReceiptCreated(File),
    ReceiptSent(ReceiptSent),
    StepCompleted(Step),
    TenantCreated(TenantCreated),
    TenantUpdated(TenantUpdated),
}

impl From<Event> for EventType {
    fn from(item: Event) -> Self {
        match item {
            Event::AdvertisementCreated(_) => unimplemented!(),
            Event::AdvertisementUpdated(_) => unimplemented!(),
            Event::CandidacyAccepted(_) => Self::CandidacyAccepted,
            Event::CandidacyCreated(_) => Self::CandidacyCreated,
            Event::CandidacyRejected(_) => Self::CandidacyRejected,
            Event::LeaseAffected(_) => unimplemented!(),
            Event::LeaseCreated(_) => Self::LeaseCreated,
            Event::NoticeCreated(_) => Self::NoticeCreated,
            Event::PaymentCreated(_) => Self::PaymentCreated,
            Event::PropertyCreated(_) => unimplemented!(),
            Event::PropertyUpdated(_) => unimplemented!(),
            Event::ReceiptCreated(_) => Self::ReceiptCreated,
            Event::ReceiptSent(_) => Self::ReceiptSent,
            Event::StepCompleted(_) => Self::StepCompleted,
            Event::TenantCreated(_) => unimplemented!(),
            Event::TenantUpdated(_) => unimplemented!(),
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
            Event::LeaseAffected(event) => lease_affected(&ctx, event.clone()),
            Event::LeaseCreated(event) => lease_created(&ctx, event.clone()),
            Event::NoticeCreated(notice) => notice_created(&ctx, event, notice),
            Event::PaymentCreated(payment) => payment_created(&ctx, event, payment),
            Event::PropertyCreated(event) => property_created(&ctx, event.clone()),
            Event::PropertyUpdated(event) => property_updated(&ctx, event.clone()),
            Event::ReceiptCreated(receipt) => receipt_created(&ctx, event, receipt),
            Event::ReceiptSent(_) => unimplemented!(),
            Event::StepCompleted(step) => step_completed(&ctx, event, step),
            Event::TenantCreated(event) => tenant_created(&ctx, event.clone()),
            Event::TenantUpdated(event) => tenant_updated(&ctx, event.clone()),
        })
    })?;

    Ok(events)
}

pub async fn dispatch_async(events: Vec<Event>) -> Result<()> {
    let ctx = Context::env();

    for event in events {
        match event {
            Event::ReceiptSent(event) => receipt_sent(&ctx, event).await?,
            _ => unimplemented!(),
        }
    }

    Ok(())
}
