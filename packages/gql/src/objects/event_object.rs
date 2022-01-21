use crate::unions::Eventable;
use async_graphql::Context;
use async_graphql::Result;
use async_graphql::SimpleObject;
use trankeel::AccountId;
use trankeel::Client;
use trankeel::DateTime;
use trankeel::EventId;
use trankeel::EventType;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Event {
    pub id: EventId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
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

impl From<trankeel::Event> for Event {
    fn from(item: trankeel::Event) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            account_id: item.account_id,
            type_: item.type_,
        }
    }
}

impl From<trankeel::EventWithEventable> for Event {
    fn from(item: trankeel::EventWithEventable) -> Self {
        item.0.into()
    }
}
