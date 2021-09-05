use async_graphql::InputObject;
use piteo_data::DateTime;
use piteo_data::LeaseId;

#[derive(InputObject)]
pub struct SendPaymentNoticeInput {
    lease_id: LeaseId,
    date: Option<DateTime>,
}
