use crate::billing::BillingProvider;
use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_data::Account;
use trankeel_ops::event::SubscriptionRequested;

pub fn subscription_requested(_ctx: &Context, _event: SubscriptionRequested) -> Result<()> {
    Ok(())
}

pub async fn subscription_requested_async(
    ctx: &Context,
    event: SubscriptionRequested,
) -> Result<()> {
    let db = ctx.db();
    let billing_provider = ctx.billing_provider();

    let SubscriptionRequested { account_id, email } = event;

    // Get account
    let account = db.accounts().by_id(&account_id)?;

    // Create subscription.
    let subscription = billing_provider
        .create_subscription_with_customer(email)
        .await?;

    // Update the local customer data.
    db.accounts().update(&Account {
        id: account.id,
        stripe_customer_id: Some(subscription.customer_id.clone()),
        stripe_subscription_id: Some(subscription.id.clone()),
        status: subscription.status,
        trial_end: subscription.trial_end,
        ..account
    })?;

    Ok(())
}
