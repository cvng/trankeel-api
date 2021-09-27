use crate::common::Id;
use crate::schema::accounts;
use crate::AccountStatus;
use crate::CustomerId;
use crate::DateTime;
use crate::PlanId;
use crate::SubscriptionId;

// # Types

pub type AccountId = Id;

#[derive(Clone, Insertable, Queryable)]
pub struct Account {
    pub id: AccountId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub plan_id: Option<PlanId>,
    pub status: Option<AccountStatus>,
    pub stripe_customer_id: Option<CustomerId>,
    pub stripe_subscription_id: Option<SubscriptionId>,
    pub trial_end: Option<DateTime>,
}

#[derive(Default, AsChangeset, Identifiable, Insertable)]
#[table_name = "accounts"]
pub struct AccountData {
    pub id: AccountId,
    pub plan_id: Option<PlanId>,
    pub status: Option<AccountStatus>,
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
