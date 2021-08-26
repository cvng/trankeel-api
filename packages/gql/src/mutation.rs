use async_graphql::Result;

pub struct Mutation;

#[async_graphql::Object]
impl Mutation {
    async fn user_create_with_account(&self) -> Result<String> {
        todo!()
    }
}
