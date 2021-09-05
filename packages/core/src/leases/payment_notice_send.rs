use piteo_data::DateTime;
use piteo_data::LeaseId;

#[derive(async_graphql::InputObject)]
pub struct SendPaymentNoticeInput {
    lease_id: LeaseId,
    date: Option<DateTime>,
}
