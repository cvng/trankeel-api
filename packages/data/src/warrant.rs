use crate::schema::professional_warrants;
use crate::schema::warrants;
use crate::DateTime;
use crate::Id;
use crate::Person;
use crate::PersonId;
use crate::TenantId;
use async_graphql::Enum;
use diesel_enum_derive::DieselEnum;
use serde::Deserialize;
use serde::Serialize;

pub type WarrantId = Id;

pub type ProfessionalWarrantId = Id;

pub type WarrantWithIdentity = (Warrant, WarrantIdentity);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DieselEnum, Enum)]
pub enum WarrantType {
    Person,
    Visale,
    Company,
}

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
    pub tenant_id: TenantId,
    pub individual_id: Option<PersonId>,
    pub professional_id: Option<ProfessionalWarrantId>,
}

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct ProfessionalWarrant {
    pub id: WarrantId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub name: String,
    pub identifier: String,
}
