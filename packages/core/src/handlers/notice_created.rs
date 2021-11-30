use crate::dispatcher::Event;
use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use crate::messenger;
use trankeel_data::Eventable;
use trankeel_data::Notice;

pub fn notice_created(ctx: &Context, event: &Event, notice: &Notice) -> Result<()> {
    let db = ctx.db();

    let account = db.accounts().by_notice_id(&notice.id)?;
    let participant = db.persons().by_notice_id(&notice.id)?;
    let eventable = db.eventables().create(&Eventable::File(notice.clone()))?;

    messenger::message(
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
