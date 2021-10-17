use crate::schema::invites;
use crate::AccountId;
use crate::AsUrl;
use crate::DateTime;
use crate::Id;
use crate::InviteToken;
use crate::PersonId;
use trankeel_kit::config::config;

pub type InviteId = Id;

#[derive(Copy, Clone, Debug, Eq, PartialEq, DbEnum, Enum)]
#[DieselType = "Invitereason"]
pub enum InviteReason {
    CandidacyAccepted,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, DbEnum, Enum)]
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

#[derive(Clone, Debug, Insertable, Queryable)]
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

impl AsUrl for Invite {
    fn web_route(&self) -> String {
        config()
            .web_routes("invite_url")
            .unwrap()
            .replace(":token", self.token.inner())
    }
}

#[derive(Default, AsChangeset, Identifiable, Insertable)]
#[table_name = "invites"]
pub struct InviteData {
    pub id: InviteId,
    pub account_id: Option<AccountId>,
    pub invitee_id: Option<PersonId>,
    pub token: Option<InviteToken>,
    pub status: Option<InviteStatus>,
    pub reason: Option<InviteReason>,
}
