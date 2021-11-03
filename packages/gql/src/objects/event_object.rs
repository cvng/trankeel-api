use crate::unions::Eventable;
use trankeel::AccountId;
use trankeel::DateTime;
use trankeel::EventId;
use trankeel::EventType;

#[derive(SimpleObject)]
pub struct Event {
    pub id: EventId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub type_: EventType,
    pub object: Eventable,
}

impl From<trankeel::PublicEvent> for Event {
    fn from(item: trankeel::PublicEvent) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            account_id: item.account_id,
            type_: item.event_type,
            object: item.eventable().unwrap().into(),
        }
    }
}
