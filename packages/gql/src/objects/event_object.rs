use async_graphql::SimpleObject;
use trankeel::DateTime;
use trankeel::EventId;
use trankeel::EventType;

#[derive(SimpleObject)]
pub struct Event {
    pub id: EventId,
    pub created_at: Option<DateTime>,
    pub type_: EventType,
}

impl From<trankeel::Event> for Event {
    fn from(item: trankeel::Event) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            type_: item.type_,
        }
    }
}
