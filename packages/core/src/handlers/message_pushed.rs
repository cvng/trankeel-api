use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use chrono::Utc;
use trankeel_data::Discussion;
use trankeel_ops::event::MessagePushed;

pub fn message_pushed(ctx: &Context, event: MessagePushed) -> Result<()> {
    let db = ctx.db();

    let MessagePushed { message } = event;

    let discussion = db.discussions().by_id(&message.discussion_id)?;

    let discussion = Discussion {
        updated_at: Some(Utc::now().into()), // Touch updated_at.
        ..discussion
    };

    db.messages().create(&message)?;
    db.discussions().update(&discussion)?;

    Ok(())
}
