use async_graphql::InputObject;
use trankeel_data::AccountId;
use trankeel_data::PlanCode;

#[derive(InputObject)]
pub struct AccountActivatePlanInput {
    id: AccountId,
    name: String,
    plan_code: PlanCode,
}
