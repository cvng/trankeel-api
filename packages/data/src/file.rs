use crate::id;
use crate::sql_schema::files;
use crate::DateTime;
use crate::ExternalId;
use crate::Lease;
use crate::LeaseFileId;
use crate::NoticeId;
use crate::ReceiptId;
use crate::Rent;
use crate::Url;
use async_graphql::Enum;
use async_graphql::SimpleObject;
use diesel_derive_enum::DbEnum;
use fake::Fake;
use serde::Deserialize;
use serde::Serialize;

// # Types

id!(FileId);

id!(AttachableId);

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

#[rustfmt::skip]
#[derive(Clone, Serialize, Deserialize, AsChangeset, Identifiable, Insertable, Queryable, SimpleObject)]
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

impl File {
    pub fn lease_document(lease: &Lease) -> Self {
        let id = LeaseFileId::new();

        Self {
            id,
            created_at: Default::default(),
            updated_at: Default::default(),
            download_url: None,
            external_id: None,
            filename: Some(lease_filename(&id, lease)),
            preview_url: None,
            status: Some(FileStatus::Pending),
            type_: FileType::LeaseDocument,
        }
    }

    pub fn notice_document(rent: &Rent) -> Self {
        let id = NoticeId::new();

        Self {
            id,
            created_at: Default::default(),
            updated_at: Default::default(),
            download_url: None,
            external_id: None,
            filename: Some(notice_filename(&id, rent)),
            preview_url: None,
            status: Some(FileStatus::Pending),
            type_: FileType::PaymentNotice,
        }
    }

    pub fn receipt_document(rent: &Rent) -> Self {
        let id = ReceiptId::new();

        Self {
            id,
            created_at: Default::default(),
            updated_at: Default::default(),
            download_url: None,
            external_id: None,
            filename: Some(receipt_filename(&id, rent)),
            preview_url: None,
            status: Some(FileStatus::Pending),
            type_: FileType::RentReceipt,
        }
    }
}

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
