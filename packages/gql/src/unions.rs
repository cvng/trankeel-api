use crate::objects::Company;
use crate::objects::FurnishedLeaseDetails;
use crate::objects::Payment;
use crate::objects::Person;
use crate::objects::Rent;

#[derive(async_graphql::Union)]
pub enum Identity {
    Individual(Person),
    Company(Company),
}

#[derive(async_graphql::Union)]
pub enum LeaseDetails {
    FurnishedLeaseDetails(FurnishedLeaseDetails),
}

#[derive(async_graphql::Union)]
pub enum Eventable {
    Rent(Rent),
    Transaction(Payment),
}
