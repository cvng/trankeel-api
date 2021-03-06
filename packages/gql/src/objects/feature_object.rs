use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Feature {
    pub available: bool,
    pub title: String,
    pub key: Option<String>,
}
