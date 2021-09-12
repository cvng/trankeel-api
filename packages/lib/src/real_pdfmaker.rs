use eyre::Error;
use piteo_core::pdfmaker::Document;
use piteo_core::pdfmaker::IntoDocument;
use piteo_core::pdfmaker::PdfMaker;

pub struct DocMaker;

impl DocMaker {
    pub fn new() -> Self {
        Self
    }
}

impl PdfMaker for DocMaker {
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
