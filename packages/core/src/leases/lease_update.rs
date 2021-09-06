use crate::files::FileInput;
use async_graphql::InputObject;
use piteo_data::FurnishedLeaseDetails;
use piteo_data::LeaseId;

#[derive(InputObject)]
pub struct LeaseFurnishedUpdateInput {
    data: Option<FurnishedLeaseDetails>,
    file: Option<FileInput>,
    id: LeaseId,
}
