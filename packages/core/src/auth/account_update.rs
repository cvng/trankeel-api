use piteo_data::AccountId;
use piteo_data::PaymentMethodId;

#[derive(async_graphql::InputObject)]
pub struct AccountUpdateInput {
    id: AccountId,
    payment_method_id: PaymentMethodId,
}
