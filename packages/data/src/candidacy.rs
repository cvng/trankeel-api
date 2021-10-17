use crate::schema::candidacies;
use crate::AdvertisementId;
use crate::AsUrl;
use crate::DateTime;
use crate::Id;
use crate::TenantId;
use trankeel_kit::config::config;

// # Types

pub type CandidacyId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DbEnum, Enum)]
#[DieselType = "Candidacystatus"]
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

#[derive(Clone, Debug, Insertable, Queryable, SimpleObject)]
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

impl AsUrl for Candidacy {
    fn web_route(&self) -> String {
        config()
            .web_routes("candidacy_url")
            .unwrap()
            .replace(":id", &self.id.to_string())
    }
}

#[derive(Default, AsChangeset, Identifiable, Insertable)]
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
