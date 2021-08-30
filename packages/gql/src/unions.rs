use crate::objects::Company;
use crate::objects::LeaseFurnishedData;
use crate::objects::Person;

#[derive(async_graphql::Union)]
pub enum Identity {
    Person(Person),
    Company(Company),
}

#[derive(async_graphql::Union)]
pub enum LeaseData {
    LeaseFurnishedData(LeaseFurnishedData),
}
