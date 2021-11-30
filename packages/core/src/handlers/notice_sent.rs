use super::notice_created::notice_created;
use crate::dispatcher::Event;
use crate::context::Context;
use crate::error::Result;
use trankeel_data::Notice;

pub fn notice_sent(ctx: &Context, event: &Event, notice: &Notice) -> Result<()> {
    notice_created(ctx, event, notice)
}
