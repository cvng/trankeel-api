use crate::schema::account;
use crate::CustomerId;
use crate::DateTime;
use crate::Id;
use crate::PlanId;
use crate::SubscriptionId;
use async_graphql::Enum;
use diesel_enum_derive::DieselEnum;
use serde::Deserialize;

// # Types

pub type AccountId = Id;

/// https://stripe.com/docs/billing/subscriptions/overview#subscription-states
#[derive(Copy, Clone, Debug, PartialEq, Eq, DieselEnum, Enum)]
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

#[derive(AsChangeset, Identifiable, Queryable)]
#[table_name = "account"]
pub struct Account {
    pub plan_id: Option<PlanId>,
    pub status: Option<AccountStatus>,
    pub stripe_customer_id: Option<CustomerId>,
    pub stripe_subscription_id: Option<SubscriptionId>,
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
