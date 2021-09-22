use super::adapter::Adapter;
use super::adapter::Resource;
use super::lib::Result;
use serde::Deserialize;
use serde::Serialize;
use serde_json::to_string as to_json;

const COMPLETE_STATUSES: [&str; 3] = ["error", "failure", "success"];

const COLLECTION: &str = "documents";

const MEMBER: &str = "document";

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Document {
    pub app_id: String,
    pub checksum: String,
    pub created_at: String,
    pub document_template_id: String,
    pub download_url: Option<String>,
    pub errors: Option<Vec<String>>,
    pub filename: Option<String>,
    pub id: String,
    pub meta: Option<String>,
    pub payload: String,
    pub preview_url: String,
    pub status: String,
    pub updated_at: String,
    #[serde(skip)]
    adapter: Adapter,
}

impl Document {
    pub async fn delete(document_id: String) -> Result<Option<Self>> {
        Self::new(document_id).remove().await
    }

    pub async fn fetch(document_id: String) -> Result<Self> {
        Self::new(document_id).reload().await
    }

    pub async fn generate_and_wait<P, M>(
        document_template_id: String,
        payload: P,
        meta: Option<M>,
    ) -> Result<Self>
    where
        P: Serialize,
        M: Serialize,
    {
        let mut document = Self::generate(document_template_id, payload, meta).await?;
        while !document.done() {
            document.reload().await?;
        }
        Ok(document)
    }

    pub async fn generate<P, M>(
        document_template_id: String,
        payload: P,
        meta: Option<M>,
    ) -> Result<Self>
    where
        P: Serialize,
        M: Serialize,
    {
        let mut document = Self::with_attributes(
            document_template_id,
            to_json(&meta)?,
            to_json(&payload)?,
            "pending".into(),
        );
        document.save().await
    }

    pub fn new(id: String) -> Self {
        Self {
            id,
            ..Default::default()
        }
    }

    fn with_attributes(
        document_template_id: String,
        meta: String,
        payload: String,
        status: String,
    ) -> Self {
        Self {
            document_template_id,
            meta: Some(meta),
            payload,
            status,
            ..Default::default()
        }
    }

    pub async fn remove(&self) -> Result<Option<Self>> {
        self.adapter.call("delete", self).await
    }

    pub fn done(&self) -> bool {
        COMPLETE_STATUSES.contains(&self.status.as_str())
    }

    pub async fn reload(&mut self) -> Result<Self> {
        let attributes = self.adapter.call("get", self).await?.unwrap();
        self.update(attributes);
        Ok(self.clone())
    }

    async fn save(&mut self) -> Result<Self> {
        let attributes = self.adapter.call("post", self).await?.unwrap();
        self.update(attributes);
        Ok(self.clone())
    }

    fn update(&mut self, new_attributes: Document) {
        *self = new_attributes;
    }
}

impl Resource for Document {
    fn id(&self) -> Option<&str> {
        Some(&self.id)
    }

    fn member(&self) -> &str {
        MEMBER
    }

    fn collection(&self) -> &str {
        COLLECTION
    }
}
