use crate::common::Id;
use crate::schema::tenants;
use crate::AccountId;
use crate::DateTime;
use crate::Email;
use crate::LeaseId;
use crate::Name;
use crate::PhoneNumber;
use async_graphql::Enum;
use serde::Deserialize;

// # Types

pub type TenantId = Id;

#[derive(Copy, Clone, PartialEq, Eq, Enum)]
pub enum TenantStatus {
    Gone,
    Late,
    New,
    Uptodate,
}

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct Tenant {
    pub id: TenantId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub apl: bool,
    pub birthdate: DateTime,
    pub birthplace: Option<String>,
    pub email: Email,
    pub first_name: String,
    pub last_name: String,
    pub note: Option<String>,
    pub phone_number: Option<PhoneNumber>,
    pub lease_id: Option<LeaseId>,
    pub visale_id: Option<String>,
}

#[derive(Deserialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "tenants"]
pub struct TenantData {
    pub id: TenantId,
    pub account_id: Option<AccountId>,
    pub apl: Option<bool>,
    pub birthdate: Option<DateTime>,
    pub birthplace: Option<String>,
    pub email: Option<Email>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub note: Option<String>,
    pub phone_number: Option<PhoneNumber>,
    pub lease_id: Option<LeaseId>,
    pub visale_id: Option<String>,
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
