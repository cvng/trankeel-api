/// Query object.
pub struct Query;

#[async_graphql::Object]
impl Query {
    async fn viewer(&self) -> String {
        "Hello, viewer!".to_string()
    }
}
