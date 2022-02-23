use crate::id;
use crate::sql_schema::warrants;
use crate::CandidacyId;
use crate::DateTime;
use crate::Person;
use crate::PersonId;
use crate::ProfessionalWarrant;
use crate::ProfessionalWarrantId;
use crate::TenantId;
use async_graphql::Enum;
use diesel_derive_enum::DbEnum;
use fake::Fake;
use serde::Deserialize;
use serde::Serialize;

id!(WarrantId);

// TODO: https://github.com/cvng/trankeel-api/pull/371
#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct WarrantWithIdentity {
    pub warrant: Warrant,
    pub identity: WarrantIdentity,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DbEnum, Enum)]
#[DieselType = "Warranttype"]
pub enum WarrantType {
    Person,
    Visale,
    Company,
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Serialize, Deserialize, Union)]
pub enum WarrantIdentity {
    Individual(Person),
    Professional(ProfessionalWarrant),
}

#[derive(Clone, Debug, Serialize, Deserialize, Insertable, Queryable, SimpleObject)]
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
