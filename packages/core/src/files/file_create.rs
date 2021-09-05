use piteo_data::FileType;

#[derive(async_graphql::InputObject)]
pub struct FileInput {
    download_url: String,
    r#type: FileType,
}
