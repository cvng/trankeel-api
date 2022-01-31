use crate::id;
use crate::Address;
use crate::Company;
use crate::CompanyId;
use crate::Name;
use crate::Person;
use crate::PersonId;
use fake::Fake;

id!(LegalIdentityId);

impl From<PersonId> for LegalIdentityId {
    fn from(item: PersonId) -> Self {
        Self(item.0)
    }
}

impl From<CompanyId> for LegalIdentityId {
    fn from(item: CompanyId) -> Self {
        Self(item.0)
    }
}

#[derive(Clone)]
pub enum LegalIdentity {
    Individual(Person),
    Company(Company),
}

impl LegalIdentity {
    pub fn id(&self) -> LegalIdentityId {
        match self {
            Self::Individual(person) => person.id.into(),
            Self::Company(company) => company.id.into(),
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

impl From<Person> for LegalIdentity {
    fn from(item: Person) -> Self {
        Self::Individual(item)
    }
}
