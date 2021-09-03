use crate::AuthId;
use crate::DateTime;
use crate::Id;
use crate::Name;
use async_graphql::Enum;

// # Types

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TenantStatus {
    Gone,
    Late,
    New,
    Uptodate,
}

#[derive(Clone, Queryable)]
pub struct Tenant {
    pub account_id: Id,
    pub apl: bool,
    pub auth_id: Option<AuthId>,
    pub birthdate: DateTime,
    pub birthplace: Option<String>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub note: Option<String>,
    pub phone_number: Option<String>,
    pub role: Option<String>,
    pub id: Id,
    pub lease_id: Option<Id>,
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
