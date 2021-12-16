use super::Pg;
use crate::database::Db;
use crate::error::Result;
use crate::messenger::Messenger;
use trankeel_data::EventId;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::Message;
use trankeel_data::MessageId;
use trankeel_data::PersonId;

#[derive(Clone)]
pub struct Messagerie(Pg);

impl Messagerie {
    pub fn init(db: Pg) -> Self {
        Self(db)
    }
}

impl Messenger for Messagerie {
    fn message(
        &self,
        type_: EventType,
        eventable: Eventable,
        sender_id: PersonId,
        participant_id: PersonId,
        content: Option<String>,
    ) -> Result<Message> {
        let db = &self.0;

        let account = db.accounts().by_person_id(&participant_id)?;
        let discussion = db.discussions().by_initiator_id(&participant_id)?;

        let event = trankeel_data::Event {
            id: EventId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            account_id: account.id,
            participant_id,
            eventable_id: eventable.id(),
            type_,
        };

        let message = Message {
            id: MessageId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            discussion_id: discussion.id,
            sender_id,
            content,
            event_id: Some(event.id),
        };

        db.eventables().create(&eventable)?;
        db.events().create(&event)?;
        db.messages().create(&message)?;

        Ok(message)
    }
}
