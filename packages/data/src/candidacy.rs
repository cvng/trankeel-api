use crate::DateTime;
use crate::Id;
use crate::TenantId;
use async_graphql::Enum;
use diesel_enum_derive::DieselEnum;

// # Types

pub type CandidacyId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, DieselEnum, Enum)]
pub enum CandidacyStatus {
    Pending,
    Rejected,
    Accepted,
}

pub struct Candidacy {
    pub id: CandidacyId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub status: CandidacyStatus,
    pub tenant_id: TenantId,
    pub move_in_date: DateTime,
    pub description: String,
}
