use async_graphql::InputObject;
use piteo_data::AccountId;
use piteo_data::PaymentMethodId;

#[derive(InputObject)]
pub struct AccountUpdateInput {
    id: AccountId,
    payment_method_id: PaymentMethodId,
}
