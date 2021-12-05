use super::candidacy_accepted::candidacy_accepted;
use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use trankeel_data::Candidacy;
use trankeel_data::Discussion;
use trankeel_data::Message;

#[derive(Clone)]
pub struct CandidacyRejected {
    pub candidacy: Candidacy,
    pub discussion: Discussion,
    pub message: Message,
}

pub fn candidacy_rejected(ctx: &Context, event: CandidacyRejected) -> Result<()> {
    let db = ctx.db();

    db.candidacies().update(&event.candidacy)?;
    db.discussions().update(&event.discussion)?;
    db.messages().create(&event.message)?;

    candidacy_accepted(
        ctx,
        &Event::CandidacyRejected(event.clone()),
        &event.candidacy,
    )
}
