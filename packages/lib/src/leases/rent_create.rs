use async_graphql::InputObject;
use trankeel_data::Amount;
use trankeel_data::DateTime;
use trankeel_data::LeaseId;

#[derive(InputObject)]
pub struct RentInput {
    amount: Amount,
    charges_amount: Option<Amount>,
    lease_id: LeaseId,
    period_end: Option<DateTime>,
    period_start: Option<DateTime>,
}
