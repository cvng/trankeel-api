use crate::dispatcher::Event;
use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use crate::messenger;
use diesel::result::Error::NotFound;
use trankeel_data::Candidacy;
use trankeel_data::Eventable;

pub fn candidacy_accepted(ctx: &Context, event: &Event, candidacy: &Candidacy) -> Result<()> {
    let db = ctx.db();

    let account = db.accounts().by_candidacy_id(&candidacy.id)?;
    let participant = db.persons().by_candidacy_id(&candidacy.id)?;
    let sender = db
        .persons()
        .by_account_id(&account.id)?
        .first()
        .cloned()
        .ok_or(NotFound)?;
    let eventable = Eventable::Candidacy(candidacy.clone()); // already created in on_candidacy_created

    messenger::message(
        db,
        event.clone().into(),
        eventable.id(),
        account.id,
        sender.id,
        participant.id,
        None,
    )?;

    Ok(())
}
