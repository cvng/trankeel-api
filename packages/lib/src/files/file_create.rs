use async_graphql::InputObject;
use piteo_data::FileType;
use piteo_data::Url;

#[derive(InputObject)]
#[graphql(name = "FileInput")]
pub struct CreateFileInput {
    download_url: Url,
    type_: FileType,
}
