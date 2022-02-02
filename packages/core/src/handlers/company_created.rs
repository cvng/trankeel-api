use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::CompanyCreated;

pub fn company_created(ctx: &Context, event: CompanyCreated) -> Result<()> {
    let db = ctx.db();

    let CompanyCreated { company } = event;

    db.companies().create(&company)?;

    Ok(())
}
