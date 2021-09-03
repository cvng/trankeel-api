use async_graphql::Enum;

/// https://stripe.com/docs/billing/subscriptions/overview#subscription-states
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionStatus {
    Active,
    Canceled,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Trialing,
    Unpaid,
}

impl From<String> for SubscriptionStatus {
    fn from(item: String) -> Self {
        match item.as_str() {
            "ACTIVE" => Self::Active,
            "CANCELED" => Self::Canceled,
            "INCOMPLETE" => Self::Incomplete,
            "INCOMPLETE_EXPIRED" => Self::IncompleteExpired,
            "PAST_DUE" => Self::PastDue,
            "TRIALING" => Self::Trialing,
            "UNPAID" => Self::Unpaid,
            _ => unimplemented!(),
        }
    }
}
