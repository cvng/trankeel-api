use crate::error::Result;
use crate::pdfmaker::IntoDocument;
use crate::pdfmaker::Pdfmaker;
use async_trait::async_trait;
use pdfmonkey;
use trankeel_data::Document;

#[derive(Clone)]
pub struct Pdfmonkey(pdfmonkey::Pdfmonkey);

impl Pdfmonkey {
    pub fn init() -> Self {
        Self(pdfmonkey::Pdfmonkey::new())
    }
}

#[async_trait]
impl Pdfmaker for Pdfmonkey {
    async fn generate(&self, document: impl IntoDocument + 'async_trait) -> Result<Document> {
        let document = pdfmonkey::Document::generate(
            document.template_id(),
            document.clone(),
            Some(document.meta()),
        )
        .await?;

        Ok(Document {
            id: document.id,
            status: document.status.into(),
            checksum: document.checksum,
            document_template_id: document.document_template_id,
            download_url: document.download_url.map(Into::into).unwrap_or_default(),
            preview_url: document.preview_url.into(),
            meta: None,
            payload: document.payload,
            errors: document.errors,
            created_at: document.created_at,
            updated_at: document.updated_at,
        })
    }
}
