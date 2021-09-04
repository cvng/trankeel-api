use crate::DateTime;
use crate::Id;
use async_graphql::Enum;

// # Types

/// https://www.pdfmonkey.io/fr/doc/api/generer-un-document
#[derive(Copy, Clone, PartialEq, Eq, Enum)]
pub enum FileStatus {
    Draft,
    Failure,
    Generating,
    Pending,
    Success,
}

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
pub enum FileType {
    PaymentNotice,
    LeaseDocument,
    RentReceipt,
}

pub struct File {
    pub created_at: Option<DateTime>,
    pub download_url: Option<String>,
    pub external_id: Option<String>,
    pub filename: Option<String>,
    pub preview_url: Option<String>,
    pub status: Option<String>,
    pub r#type: String,
    pub updated_at: Option<DateTime>,
    pub id: Id,
}
