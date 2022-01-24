use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use crate::mailer::Mailer;
use crate::messenger::Messenger;
use crate::templates::CandidacyCreatedMail;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_ops::event::CandidacyCreated;

pub fn candidacy_created(ctx: &Context, event: CandidacyCreated) -> Result<()> {
    let db = ctx.db();

    let CandidacyCreated { candidacy } = event;

    db.candidacies().create(&candidacy)?;

    Ok(())
}

pub async fn candidacy_created_async(ctx: &Context, event: CandidacyCreated) -> Result<()> {
    let db = ctx.db();
    let mailer = ctx.mailer();
    let messenger = ctx.messenger();

    let CandidacyCreated { candidacy } = event;

    let candidate = db.persons().by_candidacy_id(&candidacy.id)?;

    messenger.message(
        EventType::CandidacyCreated,
        Eventable::Candidacy(candidacy.clone()),
        candidate.id,
        candidate.id,
        None,
    )?;

    mailer
        .batch(vec![CandidacyCreatedMail::try_new(&candidacy, &candidate)?])
        .await?;

    Ok(())
}
