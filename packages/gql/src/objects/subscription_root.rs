use crate::unions2::Listenable;
use async_graphql::futures_util::Stream;
use async_graphql::futures_util::StreamExt;
use async_graphql::Context;
use async_graphql::Result;
use trankeel::AuthId;
use trankeel::Client;

pub struct Subscription;

#[async_graphql::Subscription]
impl Subscription {
    async fn listen(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = Listenable>> {
        ctx.data::<AuthId>()?;

        Ok(ctx
            .data_unchecked::<Client>()
            .listen()
            .await?
            .map(Into::into))
    }
}
