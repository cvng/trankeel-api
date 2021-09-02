use crate::objects::Company;
use crate::objects::LeaseFurnishedData;
use crate::objects::Person;

#[derive(async_graphql::Union)]
pub enum Identity {
    User(Person),
    Company(Company),
}

impl From<piteo_core::Identity> for Identity {
    fn from(item: piteo_core::Identity) -> Self {
        match item {
            piteo_core::Identity::Individual(person) => Identity::User(person.into()),
            piteo_core::Identity::Company(company) => Identity::Company(company.into()),
        }
    }
}

#[derive(async_graphql::Union)]
pub enum LeaseData {
    LeaseFurnishedData(LeaseFurnishedData),
}
