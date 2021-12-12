use crate::files::CreateFileInput;
use async_graphql::InputObject;
use trankeel_data::ImportSource;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct UploadImportInput {
    pub files: Vec<CreateFileInput>,
    pub source: ImportSource,
}
