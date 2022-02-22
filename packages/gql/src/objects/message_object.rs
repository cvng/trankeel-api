use super::Person;
use crate::unions::Eventable;
use async_graphql::Context;
use async_graphql::Result;
use async_graphql::SimpleObject;
use trankeel::Client;
use trankeel::DateTime;
use trankeel::DiscussionId;
use trankeel::EventType;
use trankeel::EventableId;
use trankeel::MessageContent;
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
    pub content: Option<MessageContent>,
    pub type_: Option<EventType>,
    pub eventable_id: Option<EventableId>,
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

    async fn eventable(&self, ctx: &Context<'_>) -> Result<Option<Eventable>> {
        Ok(self
            .eventable_id
            .map(|eventable_id| {
                ctx.data_unchecked::<Client>()
                    .eventables()
                    .by_id(&eventable_id)
                    .ok() // NotFound error is ok here.
            })
            .and_then(|eventable| eventable.map(Into::into)))
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
            type_: item.type_,
            eventable_id: item.eventable_id,
        }
    }
}
