use crate::id;
use crate::sql_schema::discussions;
use crate::AccountId;
use crate::Candidacy;
use crate::DateTime;
use crate::Lease;
use crate::Message;
use crate::PersonId;
use async_graphql::Enum;
use async_graphql::Union;
use diesel_derive_enum::DbEnum;
use fake::Fake;

id!(DiscussionId);

pub type DiscussionItemRow = (Option<Candidacy>, Option<Lease>);

pub type DiscussionWithMessages = (Discussion, Vec<Message>);

#[derive(Copy, Clone, Debug, PartialEq, Eq, DbEnum, Enum)]
#[DieselType = "Discussionstatus"]
pub enum DiscussionStatus {
    Active,
    Candidacy,
}

impl Default for DiscussionStatus {
    fn default() -> Self {
        Self::Active
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Union)]
pub enum DiscussionItem {
    Candidacy(Candidacy),
    Lease(Lease),
}

#[derive(Clone, Debug, AsChangeset, Identifiable, Insertable, Queryable)]
pub struct Discussion {
    pub id: DiscussionId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub initiator_id: PersonId,
    pub status: DiscussionStatus,
}
