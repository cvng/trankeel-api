use async_graphql::InputObject;
use trankeel_data::Amount;
use trankeel_data::DateTime;
use trankeel_data::LeaseId;
use trankeel_data::TransactionType;

#[derive(InputObject)]
pub struct TransactionInput {
    amount: Amount,
    lease_id: LeaseId,
    date: DateTime,
    type_: Option<TransactionType>,
}
