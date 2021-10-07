use super::Person;
use async_graphql::Context;
use async_graphql::Result;
use piteo::Client;
use piteo::DateTime;
use piteo::DiscussionId;
use piteo::MessageId;
use piteo::PersonId;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Message {
    pub id: MessageId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub discussion_id: DiscussionId,
    pub sender_id: PersonId,
    pub content: String,
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
}

impl From<piteo::Message> for Message {
    fn from(item: piteo::Message) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            discussion_id: item.discussion_id,
            sender_id: item.sender_id,
            content: item.content,
        }
    }
}
