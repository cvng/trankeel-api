use super::LeaseFurnishedDataInput;
use crate::files::FileInput;
use async_graphql::InputObject;
use piteo_data::LeaseId;

#[derive(InputObject)]
pub struct LeaseFurnishedUpdateInput {
    data: Option<LeaseFurnishedDataInput>,
    file: Option<FileInput>,
    id: LeaseId,
}
