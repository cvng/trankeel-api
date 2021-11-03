use crate::schema::public_events;
use crate::AccountId;
use crate::DateTime;
use crate::EventId;
use crate::EventType;
use crate::Eventable;
use crate::Id;
use serde_json::from_value;
use serde_json::Value;

pub type PublicEventId = Id;

pub type PublicEventEventable = Value;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DbEnum, Enum)]
#[DieselType = "Eventabletype"]
pub enum EventableType {
    Advertisement,
    Candidacy,
    Discussion,
    File,
    Lease,
    Message,
    Payment,
    Person,
    Rent,
    Step,
    Tenant,
    Warrant,
}

#[derive(Clone, Identifiable, Insertable, Queryable)]
pub struct PublicEvent {
    pub id: PublicEventId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub event_id: EventId,
    pub event_type: EventType,
    pub eventable_type: EventableType,
    pub eventable: PublicEventEventable,
}

impl PublicEvent {
    pub fn eventable(&self) -> Result<Eventable, serde_json::Error> {
        let eventable = self.eventable.clone();

        match self.eventable_type {
            EventableType::Advertisement => from_value(eventable).map(Eventable::Advertisement),
            EventableType::Candidacy => from_value(eventable).map(Eventable::Candidacy),
            EventableType::Discussion => from_value(eventable).map(Eventable::Discussion),
            EventableType::File => from_value(eventable).map(Eventable::File),
            EventableType::Lease => from_value(eventable).map(Eventable::Lease),
            EventableType::Message => from_value(eventable).map(Eventable::Message),
            EventableType::Payment => from_value(eventable).map(Eventable::Payment),
            EventableType::Person => from_value(eventable).map(Eventable::Person),
            EventableType::Rent => from_value(eventable).map(Eventable::Rent),
            EventableType::Step => from_value(eventable).map(Eventable::Step),
            EventableType::Tenant => from_value(eventable).map(Eventable::Tenant),
            EventableType::Warrant => from_value(eventable).map(Eventable::Warrant),
        }
    }
}
