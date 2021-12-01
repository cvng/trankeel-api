use crate::Address;
use crate::Company;
use crate::Id;
use crate::Name;
use crate::Person;

#[derive(Clone)]
pub enum LegalIdentity {
    Individual(Person),
    Company(Company),
}

impl LegalIdentity {
    pub fn id(&self) -> Id {
        match self {
            Self::Individual(person) => person.id,
            Self::Company(company) => company.id,
        }
    }

    pub fn display_name(&self) -> String {
        match self {
            Self::Individual(person) => person.display_name(),
            Self::Company(company) => company.display_name(),
        }
    }

    pub fn address(&self) -> Option<Address> {
        match self {
            Self::Individual(person) => person.address.clone(),
            Self::Company(company) => company.address.clone(),
        }
    }
}
