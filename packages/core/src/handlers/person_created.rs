use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_ops::event::PersonCreated;

pub fn person_created(ctx: &Context, event: PersonCreated) -> Result<()> {
    let db = ctx.db();

    let PersonCreated { person } = event;

    db.persons().create(&person)?;

    Ok(())
}
