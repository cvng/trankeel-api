use async_graphql::InputObject;
use piteo_data::AccountId;
use piteo_data::PlanCode;

#[derive(InputObject)]
pub struct AccountActivatePlanInput {
    id: AccountId,
    name: String,
    plan_code: PlanCode,
}
