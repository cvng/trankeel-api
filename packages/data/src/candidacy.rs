use crate::schema::candidacies;
use crate::AdvertisementId;
use crate::DateTime;
use crate::Id;
use crate::TenantId;
use async_graphql::Enum;
use diesel_enum_derive::DieselEnum;
use serde::Deserialize;
use serde::Serialize;

// # Types

pub type CandidacyId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DieselEnum, Enum)]
pub enum CandidacyStatus {
    Pending,
    Rejected,
    Accepted,
}

impl Default for CandidacyStatus {
    fn default() -> Self {
        Self::Pending
    }
}

#[derive(Clone, Debug, Insertable, Queryable)]
#[table_name = "candidacies"]
pub struct Candidacy {
    pub id: CandidacyId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub status: CandidacyStatus,
    pub advertisement_id: AdvertisementId,
    pub tenant_id: TenantId,
    pub move_in_date: DateTime,
    pub description: String,
}

#[derive(Default, Deserialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "candidacies"]
pub struct CandidacyData {
    pub id: CandidacyId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub status: Option<CandidacyStatus>,
    pub advertisement_id: Option<AdvertisementId>,
    pub tenant_id: Option<TenantId>,
    pub move_in_date: Option<DateTime>,
    pub description: Option<String>,
}
