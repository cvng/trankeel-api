use crate::objects::Company;
use crate::objects::LeaseFurnishedData;
use crate::objects::Person;

#[derive(async_graphql::Union)]
pub enum Identity {
    User(Person),
    Company(Company),
}

impl From<piteo_lib::LenderIdentity> for Identity {
    fn from(item: piteo_lib::LenderIdentity) -> Self {
        match item {
            piteo_lib::LenderIdentity::Individual(person) => Self::User(person.into()),
            piteo_lib::LenderIdentity::Company(company) => Self::Company(company.into()),
        }
    }
}

#[derive(async_graphql::Union)]
pub enum LeaseData {
    LeaseFurnishedData(LeaseFurnishedData),
}
