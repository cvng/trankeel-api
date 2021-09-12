use eyre::Error;
use piteo_core::pdfmaker::Document;
use piteo_core::pdfmaker::IntoDocument;
use piteo_core::pdfmaker::Pdfmaker;

pub struct Pdfmonkey;

impl Pdfmonkey {
    pub fn new() -> Self {
        Self
    }
}

impl Pdfmaker for Pdfmonkey {
    fn generate(&self, document: impl IntoDocument) -> Result<Document, Error> {
        println!("PdfMaker.generate: {:?}", document);

        Ok(Document {
            id: Default::default(),
            status: Default::default(),
            app_id: Default::default(),
            checksum: Default::default(),
            document_template_id: Default::default(),
            download_url: Default::default(),
            preview_url: Default::default(),
            meta: Default::default(),
            payload: Default::default(),
            errors: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
        })
    }
}
