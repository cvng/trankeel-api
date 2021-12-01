use async_graphql::InputObject;
use trankeel_data::AccountId;
use trankeel_data::PaymentMethodId;

#[derive(InputObject)]
pub struct AccountUpdateInput {
    id: AccountId,
    payment_method_id: PaymentMethodId,
}
