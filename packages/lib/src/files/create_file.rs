use async_graphql::InputObject;
use trankeel_data::FileType;
use trankeel_data::Url;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateFileInput {
    download_url: Url,
    type_: FileType,
}
