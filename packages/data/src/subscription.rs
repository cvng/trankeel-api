use crate::AccountStatus;
use crate::DateTime;
use stripe::Expandable;

type SubscriptionId = String;

type CustomerId = String;

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
            trial_end: item
                .trial_end
                .map(|trial_end| DateTime::from_timestamp(trial_end * 1000, 0)),
        }
    }
}
