#[macro_use]
extern crate async_graphql;

pub mod auth;
pub mod candidacies;
pub mod error;
pub mod event;
pub mod files;
pub mod invites;
pub mod leases;
pub mod lenders;
pub mod messaging;
pub mod properties;
pub mod tenants;
pub mod warrants;
pub mod workflows;

pub trait Command {
    type Input;
    type Payload;

    fn run(self, input: Self::Input) -> crate::error::Result<Self::Payload>;
}

pub trait DomainEvent: Clone {}
