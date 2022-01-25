use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use crate::messenger::Messenger;
use crate::templates::CandidacyRejectedMail;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyStatus;
use trankeel_data::Discussion;
use trankeel_data::DiscussionStatus;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_ops::event::CandidacyRejected;
use trankeel_ops::messaging::PushMessage;
use trankeel_ops::messaging::PushMessageInput;
use trankeel_ops::messaging::PushMessagePayload;
use trankeel_ops::Command;

pub fn candidacy_rejected(ctx: &Context, event: CandidacyRejected) -> Result<()> {
    let db = ctx.db();

    let CandidacyRejected { candidacy_id } = event;

    let candidacy = db.candidacies().by_id(&candidacy_id)?;
    let candidate = db.persons().by_candidacy_id(&candidacy.id)?;
    let account = db.accounts().by_candidacy_id(&candidacy.id)?;
    let account_owner = db.persons().by_account_id_first(&account.id)?;
    let discussion = db.discussions().by_candidacy_id(&candidacy.id)?;
    let candidacy_rejected_message = CandidacyRejectedMail::try_new(&candidate)?;

    let candidacy = Candidacy {
        status: CandidacyStatus::Rejected,
        ..candidacy
    };

    let discussion = Discussion {
        status: DiscussionStatus::default(),
        ..discussion
    };

    let PushMessagePayload {
        message,
        discussion,
    } = PushMessage::new(&discussion).run(PushMessageInput {
        discussion_id: discussion.id,
        sender_id: account_owner.id,
        message: candidacy_rejected_message.to_string(),
    })?;

    db.candidacies().update(&candidacy)?;
    db.discussions().update(&discussion)?;
    db.messages().create(&message)?;

    Ok(())
}

pub async fn candidacy_rejected_async(ctx: &Context, event: CandidacyRejected) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let CandidacyRejected { candidacy_id } = event;

    let candidacy = db.candidacies().by_id(&candidacy_id)?;
    let candidate = db.persons().by_candidacy_id(&candidacy_id)?;
    let account = db.accounts().by_candidacy_id(&candidacy_id)?;
    let account_owner = db.persons().by_account_id_first(&account.id)?;

    messenger.message(
        EventType::CandidacyRejected,
        Eventable::Candidacy(candidacy),
        account_owner.id,
        candidate.id,
        None,
    )?;

    Ok(())
}
