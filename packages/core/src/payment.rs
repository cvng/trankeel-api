use eyre::Error;
use piteo_data::AccountStatus;
use piteo_data::DateTime;
use piteo_data::Email;

type Id = String;

pub type CustomerId = String;

pub struct Subscription {
    pub id: Id,
    pub customer_id: CustomerId,
    pub status: AccountStatus,
    pub trial_end: DateTime,
}

pub trait PaymentProvider {
    fn create_subscription(&self, email: Email) -> Result<Subscription, Error>;
}
