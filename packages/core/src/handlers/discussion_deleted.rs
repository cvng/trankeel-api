use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use trankeel_data::DiscussionId;

#[derive(Clone)]
pub struct DiscussionDeleted {
    pub discussion_id: DiscussionId,
}

impl DiscussionDeleted {
    pub fn with(discussion_id: DiscussionId) -> Event {
        Event::DiscussionDeleted(Self { discussion_id })
    }
}

pub fn discussion_deleted(ctx: &Context, event: DiscussionDeleted) -> Result<()> {
    let db = ctx.db();

    db.discussions().delete(&event.discussion_id).map(|_| ())
}
