use crate::files::FileInput;
use piteo_data::ImportSource;

#[derive(async_graphql::InputObject)]
pub struct ImportInput {
    files: Vec<FileInput>,
    source: ImportSource,
}
