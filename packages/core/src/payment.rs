use eyre::Error;
use piteo_data::AccountStatus;
use piteo_data::DateTime;
use piteo_data::Email;

type Id = String;

pub type CustomerId = String;

pub struct Customer {
    pub id: Id,
    pub email: Email,
}

pub struct Subscription {
    pub id: Id,
    pub status: AccountStatus,
    pub trial_end: DateTime,
}

pub trait PaymentProvider {
    fn create_customer(&self, email: Email) -> Result<Customer, Error>;

    fn create_subscription(&self, customer_id: CustomerId) -> Result<Subscription, Error>;
}
