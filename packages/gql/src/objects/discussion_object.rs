use super::Message;
use super::Person;
use crate::unions::DiscussionItem;
use async_graphql::Context;
use async_graphql::Result;
use trankeel::AccountId;
use trankeel::Client;
use trankeel::DateTime;
use trankeel::DiscussionId;
use trankeel::DiscussionStatus;
use trankeel::PersonId;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Discussion {
    pub id: DiscussionId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub initiator_id: PersonId,
    pub status: DiscussionStatus,
}

#[async_graphql::ComplexObject]
impl Discussion {
    async fn initiator(&self, ctx: &Context<'_>) -> Result<Person> {
        Ok(ctx
            .data_unchecked::<Client>()
            .persons()
            .by_id(&self.initiator_id)?
            .into())
    }

    async fn items(&self, ctx: &Context<'_>) -> Result<Vec<DiscussionItem>> {
        Ok(ctx
            .data_unchecked::<Client>()
            .discussions()
            .related_items(&self.id)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn snippet(&self, ctx: &Context<'_>) -> Result<Option<Message>> {
        Ok(ctx
            .data_unchecked::<Client>()
            .messages()
            .by_discussion_id(&self.id)?
            .first()
            .cloned()
            .map(Into::into))
    }

    async fn messages(&self, ctx: &Context<'_>) -> Result<Vec<Message>> {
        Ok(ctx
            .data_unchecked::<Client>()
            .messages()
            .by_discussion_id(&self.id)?
            .into_iter()
            .map(Into::into)
            .collect())
    }
}

impl From<trankeel::Discussion> for Discussion {
    fn from(item: trankeel::Discussion) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            account_id: item.account_id,
            initiator_id: item.initiator_id,
            status: item.status,
        }
    }
}
