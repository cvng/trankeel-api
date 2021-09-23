use crate::common::Id;
use crate::schema::lenders;
use crate::AccountId;
use crate::Address;
use crate::Company;
use crate::CompanyId;
use crate::DateTime;
use crate::LegalIdentity;
use crate::Name;
use crate::Person;
use crate::PersonId;
use serde::Deserialize;

// # Types

pub type LenderId = Id;

#[derive(Clone, Debug)]
pub enum LenderWithIdentity {
    Individual(Lender, Person),
    Company(Lender, Company),
}

#[derive(Clone, Debug, Insertable, Queryable)]
pub struct Lender {
    pub id: LenderId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub account_id: AccountId,
    pub individual_id: Option<PersonId>,
    pub company_id: Option<CompanyId>,
}

#[derive(Deserialize, AsChangeset, Identifiable, Insertable)]
#[table_name = "lenders"]
pub struct LenderData {
    pub id: LenderId,
    pub account_id: AccountId,
    pub individual_id: Option<PersonId>,
    pub company_id: Option<CompanyId>,
}

// # Impls

impl LegalIdentity for Lender {}

impl LenderWithIdentity {
    pub fn id(&self) -> LenderId {
        match self {
            Self::Individual(lender, _) => lender.id,
            Self::Company(lender, _) => lender.id,
        }
    }

    pub fn lender(&self) -> Lender {
        match self {
            Self::Individual(lender, _) => lender.clone(),
            Self::Company(lender, _) => lender.clone(),
        }
    }

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

impl From<LenderWithIdentity> for Lender {
    fn from(item: LenderWithIdentity) -> Self {
        match item {
            LenderWithIdentity::Individual(lender, _) => lender,
            LenderWithIdentity::Company(lender, _) => lender,
        }
    }
}
