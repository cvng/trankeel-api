use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::DiscussionDeleted;

pub fn discussion_deleted(ctx: &Context, event: DiscussionDeleted) -> Result<()> {
    let db = ctx.db();

    db.discussions().delete(&event.discussion_id).map(|_| ())
}
