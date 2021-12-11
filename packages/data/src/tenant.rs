use crate::schema::tenants;
use crate::AccountId;
use crate::Balance;
use crate::Date;
use crate::DateTime;
use crate::Email;
use crate::Id;
use crate::LeaseId;
use crate::Name;
use crate::Person;
use crate::PersonId;
use crate::PhoneNumber;

pub type TenantId = Id;

pub type TenantWithBalance = (Tenant, Balance);

pub type TenantWithIdentity = (Tenant, Person);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Enum)]
pub enum TenantStatus {
    Candidate, // unused
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

#[derive(Clone, Debug, AsChangeset, Identifiable, Insertable, Queryable)]
pub struct Tenant {
    pub id: TenantId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub person_id: PersonId,
    pub birthdate: Option<Date>,
    pub birthplace: Option<String>,
    pub email: Email,
    pub first_name: String,
    pub last_name: String,
    pub note: Option<String>,
    pub phone_number: Option<PhoneNumber>,
    pub lease_id: Option<LeaseId>,
    pub is_student: Option<bool>,
}

impl Tenant {
    pub fn status(&self, balance: Balance) -> TenantStatus {
        match self.lease_id {
            Some(_) => {
                if balance.balance.is_zero() {
                    TenantStatus::Uptodate
                } else {
                    TenantStatus::Late
                }
            }
            None => TenantStatus::default(),
        }
    }
}

impl Name for Tenant {
    fn first_name(&self) -> String {
        self.first_name.clone()
    }

    fn last_name(&self) -> String {
        self.last_name.clone()
    }
}
