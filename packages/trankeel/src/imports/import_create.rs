use crate::files::CreateFileInput;
use async_graphql::InputObject;
use trankeel_data::ImportSource;

#[derive(InputObject)]
pub struct ImportInput {
    files: Vec<CreateFileInput>,
    source: ImportSource,
}
