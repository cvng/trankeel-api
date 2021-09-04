use async_trait::async_trait;
use eyre::Error;
use piteo_data::Email;
use piteo_data::Subscription;

#[async_trait]
pub trait PaymentProvider {
    async fn create_subscription_with_customer(&self, email: Email) -> Result<Subscription, Error>;
}
