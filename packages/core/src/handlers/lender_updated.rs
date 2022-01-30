use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_data::LegalIdentity;
use trankeel_ops::event::LenderUpdated;

pub fn lender_updated(ctx: &Context, event: LenderUpdated) -> Result<()> {
    let db = ctx.db();

    let LenderUpdated {
        lender: _lender,
        identity,
    } = event;

    match identity {
        LegalIdentity::Individual(person) => db.persons().update(&person)?,
        _ => unimplemented!(),
    };

    Ok(())
}
