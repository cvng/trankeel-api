use crate::objects::Company;
use crate::objects::LeaseFurnishedData;
use crate::objects::Person;

#[derive(async_graphql::Union)]
pub enum Identity {
    User(Person),
    Company(Company),
}

impl From<piteo_core::LenderIdentity> for Identity {
    fn from(item: piteo_core::LenderIdentity) -> Self {
        match item {
            piteo_core::LenderIdentity::Individual(person) => Identity::User(person.into()),
            piteo_core::LenderIdentity::Company(company) => Identity::Company(company.into()),
        }
    }
}

#[derive(async_graphql::Union)]
pub enum LeaseData {
    LeaseFurnishedData(LeaseFurnishedData),
}
