pub struct File {
    pub created_at: Option<chrono::NaiveDateTime>,
    pub download_url: Option<String>,
    pub external_id: Option<String>,
    pub filename: Option<String>,
    pub preview_url: Option<String>,
    pub status: Option<String>,
    pub r#type_: String,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub id: uuid::Uuid,
}
