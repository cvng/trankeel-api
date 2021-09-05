use piteo_data::AttachableId;
use piteo_data::FileType;

#[derive(async_graphql::InputObject)]
pub struct DocumentGenerateInput {
    id: AttachableId,
    r#type: FileType,
}
