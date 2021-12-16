use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use crate::messenger::Messenger;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::Notice;

#[derive(Clone)]
pub struct NoticeCreated {
    pub notice: Notice,
}

impl From<NoticeCreated> for Event {
    fn from(item: NoticeCreated) -> Self {
        Event::NoticeCreated(item)
    }
}

pub fn notice_created(ctx: &Context, event: NoticeCreated) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let NoticeCreated { notice } = event;

    let account = db.accounts().by_notice_id(&notice.id)?;
    let participant = db.persons().by_notice_id(&notice.id)?;
    let eventable = db.eventables().create(&Eventable::File(notice))?;

    messenger.message(
        db,
        EventType::NoticeCreated,
        eventable.id(),
        account.id,
        participant.id,
        participant.id,
        None,
    )?;

    Ok(())
}
