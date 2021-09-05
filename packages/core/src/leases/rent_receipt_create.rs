use async_graphql::InputObject;
use piteo_data::LeaseId;

#[derive(InputObject)]
pub struct RentReceiptInput {
    lease_id: LeaseId,
    send_mail: Option<bool>,
}

#[derive(InputObject)]
pub struct RentReceiptsInput {
    send_mail: Option<bool>,
}
