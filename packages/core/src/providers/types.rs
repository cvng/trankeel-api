use serde::Deserialize;
use serde::Serialize;
use trankeel_data::Document;

/// https://www.pdfmonkey.io/fr/doc/api/webhooks
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PdfmonkeyInput {
    pub document: Document,
}
