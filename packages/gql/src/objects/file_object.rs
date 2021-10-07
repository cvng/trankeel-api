use piteo::DateTime;
use piteo::ExternalId;
use piteo::FileId;
use piteo::FileStatus;
use piteo::FileType;
use piteo::Url;

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

impl From<piteo::File> for File {
    fn from(item: piteo::File) -> Self {
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
