use crate::DateTime;
use crate::Id;
use crate::Person;
use crate::TenantId;
use async_graphql::Enum;
use diesel_enum_derive::DieselEnum;

// # Types

pub type WarrantId = Id;

pub type WarrantWithIdentity = (Warrant, WarrantIdentity);

#[derive(Copy, Clone, Debug, PartialEq, Eq, DieselEnum, Enum)]
pub enum WarrantType {
    Visale,
    Person,
    Private,
}

#[derive(Clone)]
pub enum WarrantIdentity {
    Person(Person),
    WarrantCompany(WarrantCompany),
}

pub struct Warrant {
    pub id: WarrantId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub tenant_id: TenantId,
    pub type_: WarrantType,
    pub identifier: String,
}

#[derive(Clone)]
pub struct WarrantCompany {
    pub identifier: String,
}
