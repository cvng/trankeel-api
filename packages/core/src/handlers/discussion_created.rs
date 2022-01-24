use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::DiscussionCreated;

pub fn discussion_created(ctx: &Context, event: DiscussionCreated) -> Result<()> {
    let db = ctx.db();

    let DiscussionCreated { discussion } = event;

    db.discussions().create(&discussion)?;

    Ok(())
}
