use async_graphql::InputObject;
use trankeel_data::FileType;
use trankeel_data::Url;

#[derive(InputObject)]
#[graphql(name = "FileInput")]
pub struct CreateFileInput {
    download_url: Url,
    type_: FileType,
}
