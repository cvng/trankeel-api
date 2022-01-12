use trankeel_data::Document;

/// https://www.pdfmonkey.io/fr/doc/api/webhooks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfmonkeyInput {
    pub document: Document,
}
