use async_graphql::InputObject;
use piteo_data::AttachableId;
use piteo_data::FileType;

#[derive(InputObject)]
pub struct DocumentGenerateInput {
    id: AttachableId,
    r#type: FileType,
}
