use super::Message;
use super::Person;
use crate::unions::DiscussionSubject;
use async_graphql::Context;
use async_graphql::Result;
use piteo::AccountId;
use piteo::Client;
use piteo::DateTime;
use piteo::DiscussionId;
use piteo::DiscussionType;
use piteo::PersonId;
use piteo::SubjectId;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Discussion {
    pub id: DiscussionId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub initiator_id: PersonId,
    pub subject_id: Option<SubjectId>,
    pub type_: DiscussionType,
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

    async fn subject(&self, ctx: &Context<'_>) -> Result<Option<DiscussionSubject>> {
        Ok(ctx
            .data_unchecked::<Client>()
            .discussions()
            .related_subject(&self.id)?
            .map(Into::into))
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

impl From<piteo::Discussion> for Discussion {
    fn from(item: piteo::Discussion) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            account_id: item.account_id,
            initiator_id: item.initiator_id,
            subject_id: item.subject_id,
            type_: item.type_,
        }
    }
}
