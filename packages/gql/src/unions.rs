use crate::objects::Company;
use crate::objects::FurnishedLeaseDetails;
use crate::objects::Payment;
use crate::objects::Person;
use crate::objects::Rent;

#[derive(async_graphql::Union)]
pub enum LegalIdentity {
    Individual(Person),
    Company(Company),
}

#[derive(async_graphql::Union)]
pub enum Eventable {
    Rent(Rent),
    Transaction(Payment),
}

impl From<piteo::Eventable> for Eventable {
    fn from(item: piteo::Eventable) -> Self {
        match item {
            piteo::Eventable::Rent(rent) => Self::Rent(rent.into()),
            piteo::Eventable::Payment(payment) => Self::Transaction(payment.into()),
        }
    }
}

#[derive(async_graphql::Union)]
pub enum LeaseDetails {
    FurnishedLeaseDetails(FurnishedLeaseDetails),
}
