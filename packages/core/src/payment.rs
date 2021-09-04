use eyre::Error;
use piteo_data::Email;
use piteo_data::Subscription;

pub trait PaymentProvider {
    fn create_subscription_with_customer(&self, email: Email) -> Result<Subscription, Error>;
}
