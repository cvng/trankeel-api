use crate::database::Db;
use crate::error::Result;
use crate::messenger::Messenger;
use trankeel_data::AccountId;
use trankeel_data::EventId;
use trankeel_data::EventType;
use trankeel_data::EventableId;
use trankeel_data::Message;
use trankeel_data::MessageId;
use trankeel_data::PersonId;

pub struct Messagerie;

impl Messagerie {
    pub fn init() -> Self {
        Self
    }
}

impl Messenger for Messagerie {
    fn message(
        &self,
        db: &impl Db,
        type_: EventType,
        eventable_id: EventableId,
        account_id: AccountId,
        participant_id: PersonId,
        sender_id: PersonId,
        content: Option<String>,
    ) -> Result<Message> {
        let event = db.events().create(&trankeel_data::Event {
            id: EventId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            account_id,
            participant_id,
            eventable_id,
            type_,
        })?;

        let discussion = db.discussions().by_initiator_id(&participant_id)?;

        db.messages().create(&Message {
            id: MessageId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            discussion_id: discussion.id,
            sender_id,
            content,
            event_id: Some(event.id),
        })
    }
}
