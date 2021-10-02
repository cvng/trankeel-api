use crate::schema::discussions;
use crate::AccountId;
use crate::Candidacy;
use crate::DateTime;
use crate::Id;
use crate::Message;
use crate::PersonId;

pub type DiscussionId = Id;

pub type SubjectId = Id;

pub type DiscussionWithMessages = (Discussion, Vec<Message>);

#[derive(Copy, Clone, Debug, Eq, PartialEq, DieselEnum, Enum)]
pub enum DiscussionType {
    Candidacy,
    Normal,
}

pub enum DiscussionSubject {
    Candidacy(Candidacy),
}

impl Default for DiscussionType {
    fn default() -> Self {
        Self::Normal
    }
}

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct Discussion {
    pub id: DiscussionId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub initiator_id: PersonId,
    pub subject_id: Option<SubjectId>,
    pub type_: DiscussionType,
}
