use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use crate::mailer::Mailer;
use crate::messenger::Messenger;
use crate::templates::CandidacyAcceptedMail;
use diesel::result::Error::NotFound;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyStatus;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::InviteReason;
use trankeel_ops::command::Command;
use trankeel_ops::event::CandidacyAccepted;
use trankeel_ops::event::Event;
use trankeel_ops::invites::CreateInvite;
use trankeel_ops::invites::CreateInviteInput;

pub fn candidacy_accepted(ctx: &Context, event: CandidacyAccepted) -> Result<()> {
    let db = ctx.db();

    let CandidacyAccepted { candidacy_id } = event;

    let candidacy = db.candidacies().by_id(&candidacy_id)?;

    // Make given candidacy accepted.
    let candidacy = Candidacy {
        status: CandidacyStatus::Accepted,
        ..candidacy
    };

    db.candidacies().update(&candidacy)?;

    Ok(())
}

pub async fn candidacy_accepted_async(ctx: &Context, event: CandidacyAccepted) -> Result<()> {
    let db = ctx.db();
    let mailer = ctx.mailer();
    let messenger = ctx.messenger();

    let CandidacyAccepted { candidacy_id } = event;

    let candidacy = db.candidacies().by_id(&candidacy_id)?;
    let account = db.accounts().by_candidacy_id(&candidacy.id)?;
    let participant = db.persons().by_candidacy_id(&candidacy.id)?;
    let sender = db
        .persons()
        .by_account_id(&account.id)?
        .first()
        .cloned()
        .ok_or(NotFound)?;

    messenger.message(
        EventType::CandidacyAccepted,
        Eventable::Candidacy(candidacy.clone()),
        sender.id,
        participant.id,
        None,
    )?;

    // Create invite for new tenant.
    let invite = CreateInvite::new(&participant)
        .run(CreateInviteInput {
            invitee_id: participant.id,
            account_id: account.id,
            reason: InviteReason::CandidacyAccepted,
        })?
        .into_iter()
        .find_map(|event| match event {
            Event::InviteCreated(event) => Some(event.invite),
            _ => None,
        })
        .unwrap();

    mailer
        .batch(vec![CandidacyAcceptedMail::try_new(
            &candidacy,
            &participant,
            &invite,
        )?])
        .await?;

    Ok(())
}
