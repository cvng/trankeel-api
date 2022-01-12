use super::types::PdfmonkeyInput;
use crate::config;
use crate::error::Result;
use crate::pdfmaker::IntoDocument;
use crate::pdfmaker::Pdfmaker;
use chrono::Utc;
use reqwest::Client;
use serde_json::json;
use trankeel_data::Document;
use trankeel_data::FileId;
use trankeel_data::FileStatus;

#[derive(Clone)]
pub struct Pdfmonkey;

impl Pdfmonkey {
    pub fn init() -> Self {
        Self
    }
}

#[async_trait]
impl Pdfmaker for Pdfmonkey {
    async fn generate(&self, document: impl IntoDocument + 'async_trait) -> Result<Document> {
        let document = pdfmonkey_document_generate(
            document.template_id(),
            document.clone(),
            Some(document.meta()),
        )
        .await?;

        Ok(Document {
            id: document.id,
            status: document.status,
            checksum: document.checksum,
            document_template_id: document.document_template_id,
            download_url: document.download_url.map(Into::into),
            preview_url: document.preview_url,
            meta: None,
            payload: document.payload,
            errors: document.errors,
            created_at: document.created_at,
            updated_at: document.updated_at,
        })
    }
}

async fn pdfmonkey_document_generate<P, M>(
    document_template_id: String,
    payload: P,
    meta: Option<M>,
) -> Result<Document>
where
    P: serde::Serialize,
    M: serde::Serialize,
{
    let document_id = FileId::new().to_string();

    let document = Document {
        id: document_id.clone(),
        status: FileStatus::Success,
        document_template_id: document_template_id.clone(),
        payload: serde_json::to_string(&payload)?,
        checksum: Default::default(),
        download_url: None,
        preview_url: format!("{}/dev/preview/{}", config::base_url(), document_id).into(),
        meta: Some(serde_json::to_string(&meta)?),
        errors: None,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };

    let input = PdfmonkeyInput {
        document: document.clone(),
    };

    config::write_json(
        &format!("pdfmonkey/{}.json", document_id),
        &json!({
            "document_template_id": document_template_id,
            "payload": payload,
            "meta": meta,
        }),
    )?;

    Client::new()
        .post(format!("{}/webhooks/pdfmonkey", config::base_url()))
        .json(&input)
        .send()
        .await?;

    Ok(document)
}
