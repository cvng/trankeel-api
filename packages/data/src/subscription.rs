use crate::AccountStatus;
use crate::DateTime;
use stripe::Expandable;

pub type CustomerId = String; // stripe::CustomerId;

pub type SubscriptionId = String; // stripe::SubscriptionId;

pub type PaymentMethodId = String; // stripe::PaymentMethodId;

#[derive(SimpleObject)]
pub struct Subscription {
    pub id: SubscriptionId,
    pub customer_id: CustomerId,
    pub status: AccountStatus,
    pub trial_end: Option<DateTime>,
}

// # Impls

impl From<stripe::Subscription> for Subscription {
    fn from(item: stripe::Subscription) -> Self {
        Self {
            id: item.id.to_string(),
            status: item.status.into(),
            customer_id: match item.customer {
                Expandable::Id(customer_id) => customer_id.to_string(),
                Expandable::Object(customer) => customer.id.to_string(),
            },
            // https://stripe.com/docs/api/subscriptions/object#subscription_object-trial_end
            trial_end: item.trial_end.map(DateTime::new),
        }
    }
}
