use crate::database::Db;
use crate::error::Result;
use trankeel_data::AccountId;
use trankeel_data::EventType;
use trankeel_data::EventableId;
use trankeel_data::Message;
use trankeel_data::PersonId;

pub trait Messenger {
    #[allow(clippy::too_many_arguments)]
    fn message(
        &self,
        db: &impl Db,
        type_: EventType,
        eventable_id: EventableId,
        account_id: AccountId,
        sender_id: PersonId,
        participant_id: PersonId,
        content: Option<String>,
    ) -> Result<Message>;
}
