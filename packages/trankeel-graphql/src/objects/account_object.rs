use super::Plan;
use trankeel::AccountId;
use trankeel::AccountStatus;
use trankeel::CustomerId;
use trankeel::DateTime;
use trankeel::PlanId;
use trankeel::SubscriptionId;

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

impl From<trankeel::Account> for Account {
    fn from(item: trankeel::Account) -> Self {
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
