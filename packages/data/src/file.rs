use crate::common::Id;
use crate::schema::files;
use crate::DateTime;
use crate::ExternalId;
use crate::Url;

// # Types

pub type FileId = Id;

pub type AttachableId = Id;

/// https://www.pdfmonkey.io/fr/doc/api/generer-un-document
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DieselEnum, Enum)]
#[serde(rename_all = "snake_case")]
pub enum FileStatus {
    Draft,
    Failure,
    Generating,
    Pending,
    Success,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DieselEnum, Enum)]
pub enum FileType {
    PaymentNotice,
    LeaseDocument,
    RentReceipt,
}

#[derive(Clone, Insertable, Queryable)]
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

#[derive(Default, Deserialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "files"]
pub struct FileData {
    pub id: FileId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub download_url: Option<Url>,
    pub external_id: Option<ExternalId>,
    pub filename: Option<String>,
    pub preview_url: Option<Url>,
    pub status: Option<FileStatus>,
    pub type_: Option<FileType>,
}

// # Impls

impl Default for FileStatus {
    fn default() -> Self {
        Self::Pending
    }
}

impl From<String> for FileStatus {
    fn from(item: String) -> Self {
        match item.as_str() {
            "draft" => FileStatus::Draft,
            "failure" => FileStatus::Failure,
            "generating" => FileStatus::Generating,
            "pending" => FileStatus::Pending,
            "success" => FileStatus::Success,
            _ => unreachable!(),
        }
    }
}
