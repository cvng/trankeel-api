use async_graphql::InputObject;
use trankeel_data::AccountId;
use trankeel_data::PaymentMethodId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct UpdateAccountInput {
    pub id: AccountId,
    pub payment_method_id: PaymentMethodId,
}
