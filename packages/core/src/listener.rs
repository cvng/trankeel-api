use crate::context::Context;
use crate::error::Result;
use futures::stream::StreamExt;
use futures::Stream;
use sqlx::postgres::PgListener;
use trankeel_ops::event::Event;

const EVENT_CHANNEL: &str = "events";

pub async fn listen(ctx: &Context) -> Result<impl Stream<Item = Event>> {
    let config = ctx.config();

    let mut listener = PgListener::connect(&config.database_url.clone().unwrap()).await?;

    listener.listen_all(vec![EVENT_CHANNEL]).await?;

    Ok(listener
        .into_stream()
        .map(|res| serde_json::from_str(&res.unwrap().payload().to_owned()).unwrap()))
}
