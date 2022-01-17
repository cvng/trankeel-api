use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use crate::messenger::Messenger;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_ops::event::CandidacyCreated;

pub fn candidacy_created(ctx: &Context, event: CandidacyCreated) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let CandidacyCreated { candidacy } = event;

    let participant = db.persons().by_candidacy_id(&candidacy.id)?;

    messenger.message(
        EventType::CandidacyCreated,
        Eventable::Candidacy(candidacy),
        participant.id,
        participant.id,
        None,
    )?;

    Ok(())
}
