use trankeel::DateTime;
use trankeel::ExternalId;
use trankeel::FileId;
use trankeel::FileStatus;
use trankeel::FileType;
use trankeel::Url;

#[derive(SimpleObject)]
pub struct File {
    pub id: FileId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub download_url: Option<Url>,
    pub external_id: Option<ExternalId>,
    pub filename: Option<String>,
    pub preview_url: Option<Url>,
    pub status: Option<FileStatus>,
    pub type_: FileType,
}

impl From<trankeel::File> for File {
    fn from(item: trankeel::File) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            download_url: item.download_url,
            external_id: item.external_id,
            filename: item.filename,
            preview_url: item.preview_url,
            status: item.status,
            type_: item.type_,
        }
    }
}
