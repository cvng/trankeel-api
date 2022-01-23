use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::AccountCreated;

pub fn account_created(ctx: &Context, event: AccountCreated) -> Result<()> {
    let db = ctx.db();

    let AccountCreated { account } = event;

    db.accounts().create(&account)?;

    Ok(())
}
