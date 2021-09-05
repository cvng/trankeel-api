use piteo_data::AccountId;
use piteo_data::PlanCode;

#[derive(async_graphql::InputObject)]
pub struct AccountActivatePlanInput {
    id: AccountId,
    name: String,
    plan_code: PlanCode,
}
