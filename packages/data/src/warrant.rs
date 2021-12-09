use crate::schema::warrants;
use crate::CandidacyId;
use crate::DateTime;
use crate::Id;
use crate::Person;
use crate::PersonId;
use crate::ProfessionalWarrant;
use crate::ProfessionalWarrantId;
use crate::TenantId;

pub type WarrantId = Id;

pub type WarrantWithIdentity = (Warrant, WarrantIdentity);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DbEnum, Enum)]
#[DieselType = "Warranttype"]
pub enum WarrantType {
    Person,
    Visale,
    Company,
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum WarrantIdentity {
    Individual(Person),
    Professional(ProfessionalWarrant),
}

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct Warrant {
    pub id: WarrantId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub type_: WarrantType,
    pub tenant_id: Option<TenantId>,
    pub individual_id: Option<PersonId>,
    pub professional_id: Option<ProfessionalWarrantId>,
    pub candidacy_id: Option<CandidacyId>,
}
