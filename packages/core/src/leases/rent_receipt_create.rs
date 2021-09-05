use piteo_data::LeaseId;

#[derive(async_graphql::InputObject)]
pub struct RentReceiptInput {
    lease_id: LeaseId,
    send_mail: Option<bool>,
}

#[derive(async_graphql::InputObject)]
pub struct RentReceiptsInput {
    send_mail: Option<bool>,
}
