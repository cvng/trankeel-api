use super::receipt_created::receipt_created;
use crate::activity::Event;
use crate::context::Context;
use crate::error::Result;
use trankeel_data::Receipt;

pub fn receipt_sent(ctx: &Context, event: &Event, receipt: &Receipt) -> Result<()> {
    receipt_created(ctx, event, receipt)
}
