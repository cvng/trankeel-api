use crate::ExternalId;
use crate::FileStatus;
use crate::Url;
use std::fmt::Debug;

pub type DocumentTemplateId = ExternalId;

pub type DocumentId = ExternalId;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Document {
    pub id: DocumentId,
    pub status: FileStatus,
    pub document_template_id: DocumentTemplateId,
    pub payload: String,
    pub checksum: String,
    pub download_url: Option<Url>,
    pub preview_url: Url,
    pub meta: Option<String>,
    pub errors: Option<Vec<String>>,
    pub created_at: String,
    pub updated_at: String,
}
