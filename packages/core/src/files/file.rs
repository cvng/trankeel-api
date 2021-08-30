use crate::DateTime;

pub struct File {
    pub created_at: Option<DateTime>,
    pub download_url: Option<String>,
    pub external_id: Option<String>,
    pub filename: Option<String>,
    pub preview_url: Option<String>,
    pub status: Option<String>,
    pub r#type: String,
    pub updated_at: Option<DateTime>,
    pub id: uuid::Uuid,
}
