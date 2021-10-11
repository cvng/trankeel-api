use crate::schema::discussions;
use crate::AccountId;
use crate::Candidacy;
use crate::DateTime;
use crate::Id;
use crate::Message;
use crate::PersonId;

pub type DiscussionId = Id;

pub type DiscussionItemRow = (Option<Candidacy>,);

pub type SubjectId = Id;

pub type DiscussionWithMessages = (Discussion, Vec<Message>);

#[derive(Copy, Clone, Debug, Eq, PartialEq, DbEnum, Enum)]
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

pub enum DiscussionItem {
    Candidacy(Candidacy),
}

impl From<DiscussionItemRow> for DiscussionItem {
    fn from(item: DiscussionItemRow) -> Self {
        None.or_else(|| item.0.clone().map(Self::Candidacy))
            .unwrap()
    }
}

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct Discussion {
    pub id: DiscussionId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub initiator_id: PersonId,
    pub status: DiscussionStatus,
}

#[derive(Default, AsChangeset, Identifiable, Insertable)]
#[table_name = "discussions"]
pub struct DiscussionData {
    pub id: DiscussionId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: Option<AccountId>,
    pub initiator_id: Option<PersonId>,
    pub status: Option<DiscussionStatus>,
}
