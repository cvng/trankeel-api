use async_graphql::InputObject;
use trankeel_data::AccountId;
use trankeel_data::PlanCode;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct ActivateAccountPlanInput {
    pub id: AccountId,
    pub name: String,
    pub plan_code: PlanCode,
}
