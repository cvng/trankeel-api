use async_graphql::InputObject;
use trankeel_data::AttachableId;
use trankeel_data::FileType;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct GenerateDocumentInput {
    attachable_id: AttachableId,
    type_: FileType,
}
