mod adapter;
mod configuration;
mod document;
mod lib;

use super::pdfmonkey::lib as pdfmonkey;
use crate::Provider;
use async_trait::async_trait;
use eyre::Error;
use piteo_core::pdfmaker::Document;
use piteo_core::pdfmaker::IntoDocument;
use piteo_core::pdfmaker::Pdfmaker;

pub struct Pdfmonkey(pdfmonkey::Pdfmonkey);

impl Provider for Pdfmonkey {
    fn init() -> Self {
        Self(pdfmonkey::Pdfmonkey::new())
    }
}

#[async_trait]
impl Pdfmaker for Pdfmonkey {
    async fn generate(
        &self,
        document: impl IntoDocument + 'async_trait,
    ) -> Result<Document, Error> {
        println!("Pdfmaker.generate: {:?}", document);

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
            download_url: document.download_url.map(Into::into),
            preview_url: document.preview_url.into(),
            meta: None,
            payload: document.payload,
            errors: document.errors,
            created_at: document.created_at,
            updated_at: document.updated_at,
        })
    }
}
