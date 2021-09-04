use crate::wip;
use piteo_core::error::Error;
use piteo_core::payment::Customer;
use piteo_core::payment::CustomerId;
use piteo_core::payment::PaymentProvider;
use piteo_core::payment::Subscription;
use piteo_core::Email;

pub struct Stripe;

impl PaymentProvider for Stripe {
    fn create_customer(&self, _email: Email) -> Result<Customer, Error> {
        Err(wip())
    }

    fn create_subscription(&self, _customer_id: CustomerId) -> Result<Subscription, Error> {
        Err(wip())
    }
}
