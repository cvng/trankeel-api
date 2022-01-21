use crate::sql_schema::accounts;
use crate::CustomerId;
use crate::DateTime;
use crate::Id;
use crate::PlanId;
use crate::SubscriptionId;
use async_graphql::Enum;
use diesel::Insertable;
use diesel_derive_enum::DbEnum;

// # Types

pub type AccountId = Id;

/// https://stripe.com/docs/billing/subscriptions/overview
#[derive(Copy, Clone, Debug, PartialEq, Eq, DbEnum, Enum)]
#[graphql(name = "SubscriptionStatus")]
#[DieselType = "Accountstatus"]
pub enum AccountStatus {
    Active,
    Canceled,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Trialing,
    Unpaid,
}

impl Default for AccountStatus {
    fn default() -> Self {
        Self::Trialing
    }
}

#[derive(Clone, AsChangeset, Identifiable, Insertable, Queryable)]
pub struct Account {
    pub id: AccountId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub plan_id: Option<PlanId>,
    pub status: AccountStatus,
    pub stripe_customer_id: Option<CustomerId>,
    pub stripe_subscription_id: Option<SubscriptionId>,
    pub trial_end: Option<DateTime>,
}

// # Impls

impl From<stripe::SubscriptionStatus> for AccountStatus {
    fn from(item: stripe::SubscriptionStatus) -> Self {
        match item {
            stripe::SubscriptionStatus::Active => Self::Active,
            stripe::SubscriptionStatus::Canceled => Self::Canceled,
            stripe::SubscriptionStatus::Incomplete => Self::Incomplete,
            stripe::SubscriptionStatus::IncompleteExpired => Self::IncompleteExpired,
            stripe::SubscriptionStatus::PastDue => Self::PastDue,
            stripe::SubscriptionStatus::Trialing => Self::Trialing,
            stripe::SubscriptionStatus::Unpaid => Self::Unpaid,
        }
    }
}
