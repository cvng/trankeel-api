use crate::files::FileInput;
use async_graphql::InputObject;
use piteo_data::ImportSource;

#[derive(InputObject)]
pub struct ImportInput {
    files: Vec<FileInput>,
    source: ImportSource,
}
