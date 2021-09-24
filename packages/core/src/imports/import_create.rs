use crate::files::CreateFileInput;
use async_graphql::InputObject;
use piteo_data::ImportSource;

#[derive(InputObject)]
pub struct ImportInput {
    files: Vec<CreateFileInput>,
    source: ImportSource,
}
