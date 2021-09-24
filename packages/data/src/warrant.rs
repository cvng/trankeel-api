use crate::schema::warrants;
use crate::DateTime;
use crate::Id;
use crate::Person;
use crate::PersonId;
use crate::TenantId;
use async_graphql::Enum;
use diesel_enum_derive::DieselEnum;

// # Types

pub type WarrantId = Id;

pub type WarrantWithIdentity = (Warrant, WarrantIdentity);

pub type Visale = String;

pub type WarrantCompany = String;

#[derive(Copy, Clone, Debug, PartialEq, Eq, DieselEnum, Enum)]
pub enum WarrantType {
    Person,
    Visale,
    Company,
}

#[derive(Clone)]
pub enum WarrantIdentity {
    Person(Person),
    Visale(Visale),
    Company(WarrantCompany),
}

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct Warrant {
    pub id: WarrantId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub type_: WarrantType,
    pub identifier: Option<String>,
    pub person_id: Option<PersonId>,
    pub tenant_id: TenantId,
}
