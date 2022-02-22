use crate::error::Result;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::Message;
use trankeel_data::PersonId;

pub trait Messenger {
    fn message(
        &self,
        sender_id: PersonId,
        participant_id: PersonId,
        content: Option<String>,
        type_: Option<EventType>,
        eventable: Option<Eventable>,
    ) -> Result<Message>;
}
