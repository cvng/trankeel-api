use crate::sql_schema::candidacies;
use crate::AdvertisementId;
use crate::Date;
use crate::DateTime;
use crate::Id;
use crate::PersonId;
use crate::Url;
use trankeel_kit::config;

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

#[derive(Clone, Debug, AsChangeset, Identifiable, Insertable, Queryable, SimpleObject)]
#[table_name = "candidacies"]
pub struct Candidacy {
    pub id: CandidacyId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub status: CandidacyStatus,
    pub advertisement_id: AdvertisementId,
    pub person_id: PersonId,
    pub move_in_date: DateTime,
    pub description: String,
    pub birthdate: Option<Date>,
    pub birthplace: Option<String>,
    pub is_student: Option<bool>,
}

pub fn candidacy_url(candidacy: &Candidacy) -> Url {
    config::config()
        .routes("candidacy_url")
        .unwrap()
        .replace(":id", &candidacy.id.to_string())
        .into()
}
