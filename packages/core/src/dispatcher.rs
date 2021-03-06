use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use crate::handlers;
use crate::providers::Pg;
use futures::stream;
use futures::stream::StreamExt;
use futures::stream::TryStreamExt;
use trankeel_data::EventId;
use trankeel_ops::event::Event;

#[remain::check]
pub async fn dispatch(ctx: &Context, events: Vec<Event>) -> Result<()> {
    Pg::transaction(ctx.db(), || {
        events.clone().into_iter().try_for_each(|evt| {
            log::info!("Event: {}", evt);

            store_event(ctx, &evt)?;

            #[remain::sorted]
            match evt {
                Event::AccountCreated(evt) => handlers::account_created(ctx, evt),
                Event::AdvertisementCreated(evt) => handlers::advertisement_created(ctx, evt),
                Event::AdvertisementUpdated(evt) => handlers::advertisement_updated(ctx, evt),
                Event::CandidacyAccepted(evt) => handlers::candidacy_accepted(ctx, evt),
                Event::CandidacyCreated(evt) => handlers::candidacy_created(ctx, evt),
                Event::CandidacyRejected(evt) => handlers::candidacy_rejected(ctx, evt),
                Event::CompanyCreated(evt) => handlers::company_created(ctx, evt),
                Event::DiscussionCreated(evt) => handlers::discussion_created(ctx, evt),
                Event::DiscussionDeleted(evt) => handlers::discussion_deleted(ctx, evt),
                Event::DocumentGenerated(evt) => handlers::document_generated(ctx, evt),
                Event::InviteAccepted(evt) => handlers::invite_accepted(ctx, evt),
                Event::InviteCreated(evt) => handlers::invite_created(ctx, evt),
                Event::LeaseAffected(evt) => handlers::lease_affected(ctx, evt),
                Event::LeaseCreated(evt) => handlers::lease_created(ctx, evt),
                Event::LeaseDeleted(evt) => handlers::lease_deleted(ctx, evt),
                Event::LeaseFileRequested(evt) => handlers::lease_file_requested(ctx, evt),
                Event::LeaseUpdated(evt) => handlers::lease_updated(ctx, evt),
                Event::LenderCreated(evt) => handlers::lender_created(ctx, evt),
                Event::LenderUpdated(evt) => handlers::lender_updated(ctx, evt),
                Event::MessagePushed(evt) => handlers::message_pushed(ctx, evt),
                Event::NoticeCreated(evt) => handlers::notice_created(ctx, evt),
                Event::PaymentCreated(evt) => handlers::payment_created(ctx, evt),
                Event::PersonCreated(evt) => handlers::person_created(ctx, evt),
                Event::PropertyCreated(evt) => handlers::property_created(ctx, evt),
                Event::PropertyDeleted(evt) => handlers::property_deleted(ctx, evt),
                Event::PropertyUpdated(evt) => handlers::property_updated(ctx, evt),
                Event::ReceiptCreated(evt) => handlers::receipt_created(ctx, evt),
                Event::ReceiptSent(evt) => handlers::receipt_sent(ctx, evt),
                Event::StepCompleted(evt) => handlers::step_completed(ctx, evt),
                Event::StepCreated(evt) => handlers::step_created(ctx, evt),
                Event::SubscriptionRequested(evt) => handlers::subscription_requested(ctx, evt),
                Event::TenantCreated(evt) => handlers::tenant_created(ctx, evt),
                Event::TenantDeleted(evt) => handlers::tenant_deleted(ctx, evt),
                Event::TenantUpdated(evt) => handlers::tenant_updated(ctx, evt),
                Event::WarrantCreated(evt) => handlers::warrant_created(ctx, evt),
                Event::WorkflowCreated(evt) => handlers::workflow_created(ctx, evt),
            }
        })
    })?;

    stream::iter(events)
        .map(Ok)
        .try_for_each_concurrent(2, |evt| async {
            #[remain::sorted]
            match evt {
                Event::CandidacyAccepted(evt) => handlers::candidacy_accepted_async(ctx, evt).await,
                Event::CandidacyCreated(evt) => handlers::candidacy_created_async(ctx, evt).await,
                Event::CandidacyRejected(evt) => handlers::candidacy_rejected_async(ctx, evt).await,
                Event::DocumentGenerated(evt) => handlers::document_generated_async(ctx, evt).await,
                Event::LeaseAffected(evt) => handlers::lease_affected_async(ctx, evt).await,
                Event::LeaseFileRequested(evt) => {
                    handlers::lease_file_requested_async(ctx, evt).await
                }
                Event::NoticeCreated(evt) => handlers::notice_created_async(ctx, evt).await,
                Event::ReceiptCreated(evt) => handlers::receipt_created_async(ctx, evt).await,
                Event::ReceiptSent(evt) => handlers::receipt_sent_async(ctx, evt).await,
                Event::StepCompleted(evt) => handlers::step_completed_async(ctx, evt).await,
                Event::SubscriptionRequested(evt) => {
                    handlers::subscription_requested_async(ctx, evt).await
                }
                _ => Ok(()),
            }
        })
        .await?;

    Ok(())
}

fn store_event(ctx: &Context, event: &Event) -> Result<()> {
    let db = ctx.db();

    db.events().create(&trankeel_data::Event {
        id: EventId::new(),
        created_at: Default::default(),
        type_: event.event_type(),
        payload: serde_json::to_value(event).unwrap(),
    })?;

    Ok(())
}
