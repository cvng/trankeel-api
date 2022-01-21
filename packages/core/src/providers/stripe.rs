use crate::billing::BillingProvider;
use crate::error::Error;
use async_trait::async_trait;
use log::info;
use trankeel_data::Email;
use trankeel_data::Subscription;
use trankeel_kit::config::Config;

#[derive(Clone)]
pub struct Stripe {
    client: stripe::Client,
    default_price_id: String,
}

impl Stripe {
    pub fn init(config: &Config) -> Self {
        Self {
            client: stripe::Client::new(config.stripe_secret_key.clone().unwrap()),
            default_price_id: config.stripe_default_price_id.clone().unwrap(),
        }
    }
}

#[async_trait]
impl BillingProvider for Stripe {
    async fn create_subscription_with_customer(&self, email: Email) -> Result<Subscription, Error> {
        let price_id = self.default_price_id.clone();

        // https://stripe.com/docs/api/customers/create
        let customer_params = stripe::CreateCustomer {
            address: Default::default(),
            balance: Default::default(),
            coupon: Default::default(),
            description: Default::default(),
            email: Some(email.inner()), // <-
            expand: Default::default(),
            invoice_prefix: Default::default(),
            invoice_settings: Default::default(),
            metadata: Default::default(),
            name: Default::default(),
            next_invoice_sequence: Default::default(),
            payment_method: Default::default(),
            phone: Default::default(),
            preferred_locales: Default::default(),
            promotion_code: Default::default(),
            shipping: Default::default(),
            source: Default::default(),
            tax_exempt: Default::default(),
            tax_id_data: Default::default(),
        };

        let customer = stripe::Customer::create(&self.client, customer_params)
            .await
            .unwrap();

        // https://stripe.com/docs/api/subscriptions/create
        let subscription_params = stripe::CreateSubscription {
            add_invoice_items: Default::default(),
            application_fee_percent: Default::default(),
            backdate_start_date: Default::default(),
            billing_cycle_anchor: Default::default(),
            billing_thresholds: Default::default(),
            cancel_at: Default::default(),
            cancel_at_period_end: Default::default(),
            collection_method: Default::default(),
            coupon: Default::default(),
            customer: customer.id, // <-
            days_until_due: Default::default(),
            default_payment_method: Default::default(),
            default_source: Default::default(),
            default_tax_rates: Default::default(),
            expand: &["customer"],
            // https://stripe.com/docs/api/subscriptions/create#create_subscription-items
            items: Some(vec![stripe::CreateSubscriptionItems {
                billing_thresholds: None,
                metadata: Default::default(),
                price: Some(price_id), // <-
                price_data: None,
                quantity: None,
                tax_rates: None,
            }]),
            metadata: Default::default(),
            off_session: Default::default(),
            payment_behavior: Default::default(),
            pending_invoice_item_interval: Default::default(),
            promotion_code: Default::default(),
            proration_behavior: Default::default(),
            transfer_data: Default::default(),
            trial_end: Default::default(),
            trial_from_plan: Some(true), // <-
            trial_period_days: Default::default(),
        };

        let subscription = stripe::Subscription::create(&self.client, subscription_params)
            .await
            .unwrap();

        info!("Created subscription {}", subscription.id);

        Ok(subscription.into())
    }
}
