use async_graphql::InputObject;
use trankeel_data::AttachableId;
use trankeel_data::FileType;

#[derive(InputObject)]
pub struct DocumentGenerateInput {
    id: AttachableId,
    type_: FileType,
}
