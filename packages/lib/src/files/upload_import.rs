use crate::files::CreateFileInput;
use async_graphql::InputObject;
use trankeel_data::ImportSource;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct UploadImportInput {
    files: Vec<CreateFileInput>,
    source: ImportSource,
}
