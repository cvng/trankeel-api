use crate::schema::invites;
use crate::AccountId;
use crate::DateTime;
use crate::Id;
use crate::InviteToken;
use crate::PersonId;

pub type InviteId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, DbEnum, Enum)]
#[DieselType = "Invitereason"]
pub enum InviteReason {
    CandidacyAccepted,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, DbEnum, Enum)]
#[DieselType = "Invitestatus"]
pub enum InviteStatus {
    Pending,
    Accepted,
}

impl Default for InviteStatus {
    fn default() -> Self {
        Self::Pending
    }
}

#[derive(Clone, Debug, AsChangeset, Identifiable, Insertable, Queryable)]
pub struct Invite {
    pub id: InviteId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: Option<AccountId>,
    pub invitee_id: PersonId,
    pub token: InviteToken,
    pub status: InviteStatus,
    pub reason: InviteReason,
}
