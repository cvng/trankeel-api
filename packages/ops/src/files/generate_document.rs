use async_graphql::InputObject;
use trankeel_data::AttachableId;
use trankeel_data::FileType;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct GenerateDocumentInput {
    pub attachable_id: AttachableId,
    pub type_: FileType,
}
