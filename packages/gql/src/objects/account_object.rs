use super::Plan;
use piteo::AccountId;
use piteo::AccountStatus;
use piteo::CustomerId;
use piteo::DateTime;
use piteo::PlanId;
use piteo::SubscriptionId;

#[derive(SimpleObject)]
#[graphql(complex)]
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

#[async_graphql::ComplexObject]
impl Account {
    async fn plan(&self) -> Option<Plan> {
        None
    }
}

impl From<piteo::Account> for Account {
    fn from(item: piteo::Account) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            plan_id: item.plan_id,
            status: item.status,
            stripe_customer_id: item.stripe_customer_id,
            stripe_subscription_id: item.stripe_subscription_id,
            trial_end: item.trial_end,
        }
    }
}
