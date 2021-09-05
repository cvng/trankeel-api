use async_graphql::InputObject;
use piteo_data::FileType;

#[derive(InputObject)]
pub struct FileInput {
    download_url: String,
    r#type: FileType,
}
