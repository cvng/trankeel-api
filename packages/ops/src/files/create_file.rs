use async_graphql::InputObject;
use trankeel_data::FileType;
use trankeel_data::Url;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateFileInput {
    pub download_url: Url,
    pub type_: FileType,
}
