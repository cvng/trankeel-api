use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use crate::messenger::Messenger;
use trankeel_data::Candidacy;
use trankeel_data::EventType;
use trankeel_data::Eventable;

#[derive(Clone)]
pub struct CandidacyCreated {
    pub candidacy: Candidacy,
}

impl From<CandidacyCreated> for Event {
    fn from(item: CandidacyCreated) -> Self {
        Event::CandidacyCreated(item)
    }
}

pub fn candidacy_created(ctx: &Context, event: CandidacyCreated) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let CandidacyCreated { candidacy } = event;

    let account = db.accounts().by_candidacy_id(&candidacy.id)?;
    let participant = db.persons().by_candidacy_id(&candidacy.id)?;
    let eventable = db.eventables().create(&Eventable::Candidacy(candidacy))?;

    messenger.message(
        db,
        EventType::CandidacyCreated,
        eventable.id(),
        account.id,
        participant.id,
        participant.id,
        None,
    )?;

    Ok(())
}
