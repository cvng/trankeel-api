use crate::billing::BillingProvider;
use crate::error::Result;
use async_trait::async_trait;
use trankeel_data::AccountStatus;
use trankeel_data::Email;
use trankeel_data::Id;
use trankeel_data::Subscription;
use trankeel_kit::config::Config;

#[derive(Clone)]
pub struct Stripe;

impl Stripe {
    #[allow(dead_code)]
    pub fn init(_config: &Config) -> Self {
        Self
    }
}

#[async_trait]
impl BillingProvider for Stripe {
    async fn create_subscription_with_customer(&self, _email: Email) -> Result<Subscription> {
        Ok(Subscription {
            id: Id::new().to_string(),
            customer_id: Id::new().to_string(),
            status: AccountStatus::Trialing,
            trial_end: None,
        })
    }
}
