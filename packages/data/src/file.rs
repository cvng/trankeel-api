use crate::schema::files;
use crate::DateTime;
use crate::ExternalId;
use crate::Id;
use crate::Lease;
use crate::Rent;
use crate::Url;

// # Types

pub type FileId = Id;

pub type AttachableId = Id;

/// https://www.pdfmonkey.io/fr/doc/api/generer-un-document
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DbEnum, Enum)]
#[DieselType = "Filestatus"]
#[serde(rename_all = "snake_case")]
pub enum FileStatus {
    Draft,
    Failure,
    Generating,
    Pending,
    Success,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DbEnum, Enum)]
#[DieselType = "Filetype"]
pub enum FileType {
    LeaseDocument,
    PaymentNotice,
    RentReceipt,
}

#[derive(Clone, AsChangeset, Identifiable, Insertable, Queryable, SimpleObject)]
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

// # Utils

/// Ex: "07-21-bail-ab60265a.pdf"
pub fn lease_filename(file_id: &FileId, lease: &Lease) -> String {
    let id = file_id.to_string();
    let id = id.split('-').next().unwrap_or("id");
    let date = lease.effect_date.inner().to_rfc3339();
    let date = date.split('T').next().unwrap_or("date");
    format!("{}-bail-{}.pdf", &date, &id) // TODO: localize
}

/// Ex: "07-21-avis-ab60265a.pdf"
pub fn notice_filename(file_id: &FileId, rent: &Rent) -> String {
    let id = file_id.to_string();
    let id = id.split('-').next().unwrap_or("id");
    let date = rent.period_start.inner().to_rfc3339();
    let date = date.split('T').next().unwrap_or("date");
    format!("{}-avis-{}.pdf", &date, &id) // TODO: localize
}

/// Ex: "07-21-quittance-ab60265a.pdf"
pub fn receipt_filename(file_id: &FileId, rent: &Rent) -> String {
    let id = file_id.to_string();
    let id = id.split('-').next().unwrap_or("id");
    let date = rent.period_start.inner().to_rfc3339();
    let date = date.split('T').next().unwrap_or("date");
    format!("{}-quittance-{}.pdf", &date, &id) // TODO: localize
}
