use crate::wip;
use piteo_core::error::Error;
use piteo_core::payment::PaymentProvider;
use piteo_core::payment::Subscription;
use piteo_core::Email;

pub struct Stripe;

impl PaymentProvider for Stripe {
    fn create_subscription(&self, _email: Email) -> Result<Subscription, Error> {
        Err(wip())
    }
}
