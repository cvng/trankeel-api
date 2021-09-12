use eyre::Error;
use piteo_data::ExternalId;
use piteo_data::FileStatus;
use piteo_data::Url;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;

pub type DocumentTemplateId = ExternalId;

pub type DocumentId = ExternalId;

pub trait Pdfmaker {
    fn generate(&self, document: impl IntoDocument) -> Result<Document, Error>;
}

pub trait IntoDocument: Serialize + Clone + fmt::Debug {
    fn template_id(&self) -> String;
    fn filename(&self) -> String;
    fn data(&'static self) -> Box<dyn erased_serde::Serialize> {
        Box::new(self.clone())
    }
    fn meta(&self) -> Meta {
        Meta {
            _filename: self.filename(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Document {
    pub id: DocumentId,
    pub status: FileStatus,
    pub app_id: String,
    pub document_template_id: DocumentTemplateId,
    pub payload: String,
    pub checksum: String,
    pub download_url: Option<Url>,
    pub preview_url: Option<Url>,
    pub meta: Option<Meta>,
    pub errors: Option<Vec<String>>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Meta {
    pub _filename: String,
}
