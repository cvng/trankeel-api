use crate::context::Context;
use crate::error::Result;
use crate::handlers::*;
use crate::providers::Pg;
use trankeel_data::Candidacy;
use trankeel_data::EventType;
use trankeel_data::File;
use trankeel_data::Payment;

pub trait Command {
    type Input;
    type Payload;

    fn run(self, input: Self::Input) -> Result<Self::Payload>;
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum Event {
    AdvertisementCreated(AdvertisementCreated),
    AdvertisementUpdated(AdvertisementUpdated),
    CandidacyAccepted(CandidacyAccepted),
    CandidacyCreated(Candidacy),
    CandidacyRejected(CandidacyRejected),
    DocumentGenerated(DocumentGenerated),
    LeaseAffected(LeaseAffected),
    LeaseCreated(LeaseCreated),
    NoticeCreated(File),
    PaymentCreated(Payment),
    PropertyCreated(PropertyCreated),
    PropertyUpdated(PropertyUpdated),
    ReceiptCreated(File),
    ReceiptSent(ReceiptSent),
    StepCompleted(StepCompleted),
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
            Event::DocumentGenerated(_) => unimplemented!(),
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

pub fn dispatch(ctx: &Context, events: Vec<Event>) -> Result<()> {
    Pg::transaction(ctx.db(), || {
        events.iter().try_for_each(|event| match event {
            Event::AdvertisementCreated(event) => advertisement_created(ctx, event.clone()),
            Event::AdvertisementUpdated(event) => advertisement_updated(ctx, event.clone()),
            Event::CandidacyAccepted(_) => unimplemented!(),
            Event::CandidacyCreated(candidacy) => candidacy_created(ctx, event, candidacy),
            Event::CandidacyRejected(event) => candidacy_rejected(ctx, event.clone()),
            Event::DocumentGenerated(_) => unimplemented!(),
            Event::LeaseAffected(event) => lease_affected(ctx, event.clone()),
            Event::LeaseCreated(event) => lease_created(ctx, event.clone()),
            Event::NoticeCreated(notice) => notice_created(ctx, event, notice),
            Event::PaymentCreated(payment) => payment_created(ctx, event, payment),
            Event::PropertyCreated(event) => property_created(ctx, event.clone()),
            Event::PropertyUpdated(event) => property_updated(ctx, event.clone()),
            Event::ReceiptCreated(receipt) => receipt_created(ctx, event, receipt),
            Event::ReceiptSent(_) => unimplemented!(),
            Event::StepCompleted(event) => step_completed(ctx, event.clone()),
            Event::TenantCreated(event) => tenant_created(ctx, event.clone()),
            Event::TenantUpdated(event) => tenant_updated(ctx, event.clone()),
        })
    })?;

    Ok(())
}

#[async_recursion]
pub async fn dispatch_async(ctx: &Context, events: Vec<Event>) -> Result<()> {
    for event in events {
        match event {
            Event::CandidacyAccepted(event) => candidacy_accepted(ctx, event).await?,
            Event::DocumentGenerated(event) => document_generated(ctx, event).await?,
            Event::ReceiptSent(event) => receipt_sent(ctx, event).await?,
            _ => unimplemented!(),
        }
    }

    Ok(())
}
