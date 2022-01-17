use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use crate::messenger::Messenger;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_ops::event::CandidacyRejected;

pub fn candidacy_rejected(ctx: &Context, event: CandidacyRejected) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let CandidacyRejected {
        candidacy,
        discussion,
        message,
    } = event;

    db.candidacies().update(&candidacy)?;
    db.discussions().update(&discussion)?;
    db.messages().create(&message)?;

    let account = db.accounts().by_candidacy_id(&candidacy.id)?;
    let participant = db.persons().by_candidacy_id(&candidacy.id)?;
    let sender = db.persons().by_account_id_first(&account.id)?;

    messenger.message(
        EventType::CandidacyRejected,
        Eventable::Candidacy(candidacy),
        sender.id,
        participant.id,
        None,
    )?;

    Ok(())
}
