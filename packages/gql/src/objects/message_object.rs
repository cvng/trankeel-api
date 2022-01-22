use super::Event;
use super::Person;
use async_graphql::Context;
use async_graphql::Result;
use async_graphql::SimpleObject;
use trankeel::Client;
use trankeel::DateTime;
use trankeel::DiscussionId;
use trankeel::EventId;
use trankeel::MessageId;
use trankeel::PersonId;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Message {
    pub id: MessageId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub discussion_id: DiscussionId,
    pub sender_id: PersonId,
    pub content: Option<String>,
    pub event_id: Option<EventId>,
}

#[async_graphql::ComplexObject]
impl Message {
    async fn sender(&self, ctx: &Context<'_>) -> Result<Person> {
        Ok(ctx
            .data_unchecked::<Client>()
            .persons()
            .by_id(&self.sender_id)?
            .into())
    }

    async fn event(&self, ctx: &Context<'_>) -> Result<Option<Event>> {
        Ok(self
            .event_id
            .map(|event_id| {
                ctx.data_unchecked::<Client>()
                    .events()
                    .by_id(&event_id)
                    .ok()
            })
            .and_then(|event| event.map(Into::into)))
    }
}

impl From<trankeel::Message> for Message {
    fn from(item: trankeel::Message) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            discussion_id: item.discussion_id,
            sender_id: item.sender_id,
            content: item.content,
            event_id: item.event_id,
        }
    }
}
