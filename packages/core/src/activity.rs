use crate::database::Db;
use crate::error::Error;
use piteo_data::AuthId;
use piteo_data::Event;
use piteo_data::EventId;
use piteo_data::EventType;
use piteo_data::EventableId;

pub fn trace(
    db: &impl Db,
    auth_id: &AuthId,
    event_type: EventType,
    eventable_id: EventableId,
) -> Result<Event, Error> {
    let account = db.accounts().by_auth_id(auth_id)?;

    db.events().create(Event {
        id: EventId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        account_id: account.id,
        eventable_id,
        eventable_type: event_type.into(),
        type_: event_type,
    })
}
