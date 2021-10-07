use crate::error::Error;
use async_trait::async_trait;
use piteo_data::Document;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Debug;

#[async_trait]
pub trait Pdfmaker {
    async fn generate(&self, document: impl IntoDocument + 'async_trait)
        -> Result<Document, Error>;
}

pub trait IntoDocument: Serialize + Clone + Debug + Send + Sync {
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Meta {
    pub _filename: String,
}
