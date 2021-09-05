use crate::common::Id;
use crate::schema::lender;
use crate::AccountId;
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
    Individual(Person),
    Company(Company),
}

#[derive(Clone, Queryable)]
pub struct Lender {
    pub id: LenderId,
    pub account_id: Id,
    pub individual_id: Option<Id>,
    pub company_id: Option<Id>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "lender"]
pub struct LenderData {
    pub account_id: AccountId,
    pub individual_id: Option<PersonId>,
    pub company_id: Option<CompanyId>,
}

// # Impls

impl LegalEntity for Lender {}

impl LenderIdentity {
    pub fn display_name(&self) -> String {
        match self {
            Self::Individual(person) => person.display_name(),
            Self::Company(company) => company.display_name(),
        }
    }
}
