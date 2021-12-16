use crate::context::Context;
use crate::error::Result;
use crate::handlers;
use crate::providers::Pg;
use futures::stream;
use futures::stream::StreamExt;
use futures::stream::TryStreamExt;

pub trait Command {
    type Input;
    type Payload;

    fn run(self, input: Self::Input) -> Result<Self::Payload>;
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum Event {
    AdvertisementCreated(handlers::AdvertisementCreated),
    AdvertisementUpdated(handlers::AdvertisementUpdated),
    CandidacyAccepted(handlers::CandidacyAccepted),
    CandidacyCreated(handlers::CandidacyCreated),
    CandidacyRejected(handlers::CandidacyRejected),
    DocumentGenerated(handlers::DocumentGenerated),
    LeaseAffected(handlers::LeaseAffected),
    LeaseCreated(handlers::LeaseCreated),
    NoticeCreated(handlers::NoticeCreated),
    PaymentCreated(handlers::PaymentCreated),
    PropertyCreated(handlers::PropertyCreated),
    PropertyUpdated(handlers::PropertyUpdated),
    ReceiptCreated(handlers::ReceiptCreated),
    ReceiptSent(handlers::ReceiptSent),
    StepCompleted(handlers::StepCompleted),
    TenantCreated(handlers::TenantCreated),
    TenantUpdated(handlers::TenantUpdated),
}

pub async fn dispatch(ctx: &Context, events: Vec<Event>) -> Result<()> {
    Pg::transaction(ctx.db(), || {
        events.clone().into_iter().try_for_each(|evt| match evt {
            Event::AdvertisementCreated(evt) => handlers::advertisement_created(ctx, evt),
            Event::AdvertisementUpdated(evt) => handlers::advertisement_updated(ctx, evt),
            Event::CandidacyAccepted(evt) => handlers::candidacy_accepted(ctx, evt),
            Event::CandidacyCreated(evt) => handlers::candidacy_created(ctx, evt),
            Event::CandidacyRejected(evt) => handlers::candidacy_rejected(ctx, evt),
            Event::DocumentGenerated(evt) => handlers::document_generated(ctx, evt),
            Event::LeaseAffected(evt) => handlers::lease_affected(ctx, evt),
            Event::LeaseCreated(evt) => handlers::lease_created(ctx, evt),
            Event::NoticeCreated(evt) => handlers::notice_created(ctx, evt),
            Event::PaymentCreated(evt) => handlers::payment_created(ctx, evt),
            Event::PropertyCreated(evt) => handlers::property_created(ctx, evt),
            Event::PropertyUpdated(evt) => handlers::property_updated(ctx, evt),
            Event::ReceiptCreated(evt) => handlers::receipt_created(ctx, evt),
            Event::ReceiptSent(evt) => handlers::receipt_sent(ctx, evt),
            Event::StepCompleted(evt) => handlers::step_completed(ctx, evt),
            Event::TenantCreated(evt) => handlers::tenant_created(ctx, evt),
            Event::TenantUpdated(evt) => handlers::tenant_updated(ctx, evt),
        })
    })?;

    stream::iter(events)
        .map(Ok)
        .try_for_each_concurrent(2, |evt| async {
            match evt {
                Event::CandidacyAccepted(evt) => handlers::candidacy_accepted_async(ctx, evt).await,
                Event::DocumentGenerated(evt) => handlers::document_generated_async(ctx, evt).await,
                Event::NoticeCreated(evt) => handlers::notice_created_async(ctx, evt).await,
                Event::ReceiptCreated(evt) => handlers::receipt_created_async(ctx, evt).await,
                Event::ReceiptSent(evt) => handlers::receipt_sent_async(ctx, evt).await,
                _ => Ok(()),
            }
        })
        .await?;

    Ok(())
}
