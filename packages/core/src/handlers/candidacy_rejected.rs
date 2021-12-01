use super::candidacy_accepted::candidacy_accepted;
use crate::context::Context;
use crate::dispatcher::Event;
use crate::error::Result;
use trankeel_data::Candidacy;

pub fn candidacy_rejected(ctx: &Context, event: &Event, candidacy: &Candidacy) -> Result<()> {
    candidacy_accepted(ctx, event, candidacy)
}
