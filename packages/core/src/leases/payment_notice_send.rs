use async_graphql::InputObject;
use piteo_data::DateTime;
use piteo_data::RentId;

#[derive(InputObject)]
pub struct SendPaymentNoticeInput {
    rent_ids: Vec<RentId>,
    date: Option<DateTime>,
}
