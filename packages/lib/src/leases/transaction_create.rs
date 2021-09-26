use async_graphql::InputObject;
use piteo_data::Amount;
use piteo_data::DateTime;
use piteo_data::LeaseId;
use piteo_data::TransactionType;

#[derive(InputObject)]
pub struct TransactionInput {
    amount: Amount,
    lease_id: LeaseId,
    date: DateTime,
    r#type: Option<TransactionType>,
}
