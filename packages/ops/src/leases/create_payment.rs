use async_graphql::InputObject;
use trankeel_data::Amount;
use trankeel_data::DateTime;
use trankeel_data::RentId;
use trankeel_data::TransactionType;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreatePaymentInput {
    rent_id: RentId,
    amount: Amount,
    date: DateTime,
    type_: TransactionType,
    label: Option<String>,
}
