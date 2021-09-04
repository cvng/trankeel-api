use crate::schema::account;
use crate::DateTime;
use crate::Id;
use crate::PlanId;
use async_graphql::Enum;
use serde::Deserialize;

// # Types

pub type AccountId = Id;

/// https://stripe.com/docs/billing/subscriptions/overview#subscription-states
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(name = "SubscriptionStatus")]
pub enum AccountStatus {
    Active,
    Canceled,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Trialing,
    Unpaid,
}

#[derive(Queryable)]
pub struct Account {
    pub plan_id: Option<PlanId>,
    pub status: Option<String>,
    pub stripe_customer_id: Option<String>,
    pub stripe_subscription_id: Option<String>,
    pub trial_end: Option<DateTime>,
    pub owner_id: String, // TODO: PersonId,
    pub id: AccountId,
}

#[derive(Deserialize, Insertable)]
#[table_name = "account"]
pub struct AccountData {
    pub owner_id: String, // TODO: PersonId,
}

// # Impls

impl From<String> for AccountStatus {
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
