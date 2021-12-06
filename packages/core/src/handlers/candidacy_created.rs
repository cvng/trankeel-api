use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use crate::messenger::Messenger;
use trankeel_data::Candidacy;
use trankeel_data::Eventable;

pub fn candidacy_created(ctx: &Context, event: &Event, candidacy: &Candidacy) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let account = db.accounts().by_candidacy_id(&candidacy.id)?;
    let participant = db.persons().by_candidacy_id(&candidacy.id)?;
    let eventable = db
        .eventables()
        .create(&Eventable::Candidacy(candidacy.clone()))?;

    messenger.message(
        db,
        event.clone().into(),
        eventable.id(),
        account.id,
        participant.id,
        participant.id,
        None,
    )?;

    Ok(())
}
