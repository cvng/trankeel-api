use super::LeaseFurnishedDataInput;
use crate::files::FileInput;
use piteo_data::LeaseId;

#[derive(async_graphql::InputObject)]
pub struct LeaseFurnishedUpdateInput {
    data: Option<LeaseFurnishedDataInput>,
    file: Option<FileInput>,
    id: LeaseId,
}
