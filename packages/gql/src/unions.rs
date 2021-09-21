use crate::objects::Company;
use crate::objects::FurnishedLeaseDetails;
use crate::objects::Person;
use crate::objects::Rent;
use crate::objects::Transaction;

#[derive(async_graphql::Union)]
pub enum Identity {
    Individual(Person),
    Company(Company),
}

impl From<piteo::LenderIdentity> for Identity {
    fn from(item: piteo::LenderIdentity) -> Self {
        match item {
            piteo::LenderIdentity::Individual(_, person) => Self::Individual(person.into()),
            piteo::LenderIdentity::Company(_, company) => Self::Company(company.into()),
        }
    }
}

#[derive(async_graphql::Union)]
pub enum LeaseDetails {
    FurnishedLeaseDetails(FurnishedLeaseDetails),
}

#[derive(async_graphql::Union)]
pub enum Eventable {
    Rent(Rent),
    Transaction(Transaction),
}
