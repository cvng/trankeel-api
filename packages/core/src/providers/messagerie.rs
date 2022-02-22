use super::Pg;
use crate::database::Db;
use crate::error::Result;
use crate::messenger::Messenger;
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
        sender_id: PersonId,
        participant_id: PersonId,
        content: Option<String>,
        type_: Option<EventType>,
        eventable: Option<Eventable>,
    ) -> Result<Message> {
        let db = &self.0;

        let discussion = db.discussions().by_initiator_id(&participant_id)?;

        let mut message = Message {
            id: MessageId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            discussion_id: discussion.id,
            sender_id,
            content,
            type_,
            eventable_id: None,
        };

        if let Some(eventable) = eventable {
            message.eventable_id = Some(eventable.id());
            db.eventables().create(&eventable)?;
        }

        db.messages().create(&message)?;

        Ok(message)
    }
}
