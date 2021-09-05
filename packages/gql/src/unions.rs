use crate::objects::Company;
use crate::objects::LeaseFurnishedData;
use crate::objects::Person;

#[derive(async_graphql::Union)]
pub enum Identity {
    User(Person),
    Company(Company),
}

impl From<piteo::LenderIdentity> for Identity {
    fn from(item: piteo::LenderIdentity) -> Self {
        match item {
            piteo::LenderIdentity::Individual(person) => Self::User(person.into()),
            piteo::LenderIdentity::Company(company) => Self::Company(company.into()),
        }
    }
}

#[derive(async_graphql::Union)]
pub enum LeaseData {
    LeaseFurnishedData(LeaseFurnishedData),
}
