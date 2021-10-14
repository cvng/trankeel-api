use crate::unions::Eventable;
use async_graphql::Context;
use async_graphql::Result;
use piteo::AccountId;
use piteo::Client;
use piteo::DateTime;
use piteo::EventId;
use piteo::EventType;
use piteo::EventableId;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Event {
    pub id: EventId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub eventable_id: EventableId,
    pub type_: EventType,
}

#[async_graphql::ComplexObject]
impl Event {
    #[graphql(name = "object")]
    async fn eventable(&self, ctx: &Context<'_>) -> Result<Eventable> {
        Ok(ctx
            .data_unchecked::<Client>()
            .events()
            .by_id(&self.id)?
            .into())
    }
}

impl From<piteo::Event> for Event {
    fn from(item: piteo::Event) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            account_id: item.account_id,
            eventable_id: item.eventable_id,
            type_: item.type_,
        }
    }
}

impl From<piteo::EventWithEventable> for Event {
    fn from(item: piteo::EventWithEventable) -> Self {
        item.0.into()
    }
}
