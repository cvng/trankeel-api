use crate::error::Error;
use trankeel_data::Email;
use trankeel_data::Subscription;

#[async_trait]
pub trait BillingProvider {
    async fn create_subscription_with_customer(&self, email: Email) -> Result<Subscription, Error>;
}
