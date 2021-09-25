use crate::common::Id;
use crate::schema::tenants;
use crate::AccountId;
use crate::Date;
use crate::DateTime;
use crate::Email;
use crate::LeaseId;
use crate::Name;
use crate::PhoneNumber;
use async_graphql::Enum;
use diesel_enum_derive::DieselEnum;
use serde::Deserialize;
use serde::Serialize;

// # Types

pub type TenantId = Id;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DieselEnum, Enum)]
pub enum TenantStatus {
    Candidate,
    Gone,
    Late,
    New,
    Uptodate,
}

impl Default for TenantStatus {
    fn default() -> Self {
        Self::New
    }
}

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct Tenant {
    pub id: TenantId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub apl: Option<bool>,
    pub birthdate: Date,
    pub birthplace: Option<String>,
    pub email: Email,
    pub first_name: String,
    pub last_name: String,
    pub note: Option<String>,
    pub phone_number: Option<PhoneNumber>,
    pub status: TenantStatus,
    pub lease_id: Option<LeaseId>,
    pub is_student: Option<bool>,
}

#[derive(Default, Deserialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "tenants"]
pub struct TenantData {
    pub id: TenantId,
    pub account_id: Option<AccountId>,
    pub apl: Option<bool>,
    pub birthdate: Option<Date>,
    pub birthplace: Option<String>,
    pub email: Option<Email>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub note: Option<String>,
    pub phone_number: Option<PhoneNumber>,
    pub status: Option<TenantStatus>,
    pub lease_id: Option<LeaseId>,
    pub is_student: Option<bool>,
}

// # Impls

impl Name for Tenant {
    fn first_name(&self) -> String {
        self.first_name.clone()
    }

    fn last_name(&self) -> String {
        self.last_name.clone()
    }
}
