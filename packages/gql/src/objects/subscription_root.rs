use async_graphql::async_stream;
use async_graphql::futures_util::Stream;
use async_graphql::Context;
use async_graphql::Result;
use std::time::Duration;
use trankeel::AuthId;

pub struct Subscription;

#[async_graphql::Subscription]
impl Subscription {
    async fn viewer(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = Option<AuthId>>> {
        let value = ctx.data_opt::<AuthId>().cloned();

        Ok(async_stream::stream! {
            loop {
                futures_timer::Delay::new(Duration::from_secs(1)).await;

                yield value.clone();
            }
        })
    }
}
