use crate::common::Id;
use crate::schema::lender;
use crate::AccountId;
use crate::Address;
use crate::Company;
use crate::CompanyId;
use crate::LegalEntity;
use crate::Name;
use crate::Person;
use crate::PersonId;
use serde::Deserialize;

// # Types

pub type LenderId = Id;

pub enum LenderIdentity {
    Individual(Lender, Person),
    Company(Lender, Company),
}

#[derive(Clone, Insertable, Queryable)]
#[table_name = "lender"]
pub struct Lender {
    pub id: LenderId,
    pub account_id: AccountId,
    pub individual_id: Option<PersonId>,
    pub company_id: Option<CompanyId>,
}

#[derive(Deserialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "lender"]
pub struct LenderData {
    pub id: LenderId,
    pub account_id: AccountId,
    pub individual_id: Option<PersonId>,
    pub company_id: Option<CompanyId>,
}

// # Impls

impl LegalEntity for Lender {}

impl LenderIdentity {
    pub fn display_name(&self) -> String {
        match self {
            Self::Individual(_, person) => person.display_name(),
            Self::Company(_, company) => company.display_name(),
        }
    }

    pub fn address(&self) -> Option<Address> {
        match self {
            Self::Individual(_, person) => person.address.clone(),
            Self::Company(_, company) => company.address.clone(),
        }
    }
}

impl From<LenderIdentity> for Lender {
    fn from(item: LenderIdentity) -> Self {
        match item {
            LenderIdentity::Individual(lender, _) => lender,
            LenderIdentity::Company(lender, _) => lender,
        }
    }
}
