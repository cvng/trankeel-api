use crate::Company;
use crate::Id;
use crate::LegalEntity;
use crate::Name;
use crate::Person;

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
