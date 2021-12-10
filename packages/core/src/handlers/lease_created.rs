use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use trankeel_data::Lease;
use trankeel_data::Rent;

#[derive(Clone)]
pub struct LeaseCreated {
    pub lease: Lease,
    pub rents: Vec<Rent>,
}

impl From<LeaseCreated> for Event {
    fn from(item: LeaseCreated) -> Self {
        Self::LeaseCreated(item)
    }
}

pub fn lease_created(ctx: &Context, event: LeaseCreated) -> Result<()> {
    let db = ctx.db();

    let LeaseCreated { lease, rents } = event;

    db.leases().create(&lease)?;
    db.rents().create_many(&rents)?;

    Ok(())
}
