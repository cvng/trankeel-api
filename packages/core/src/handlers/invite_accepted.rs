use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_data::Invite;
use trankeel_data::InviteStatus;
use trankeel_data::Person;
use trankeel_ops::event::InviteAccepted;

pub fn invite_accepted(ctx: &Context, event: InviteAccepted) -> Result<()> {
    let db = ctx.db();

    let InviteAccepted { invite_id, auth_id } = event;

    let invite = db.invites().by_id(&invite_id)?;
    let invitee = db.persons().by_id(&invite.invitee_id)?;

    // Attach user with account.
    let invitee = Person {
        id: invite.invitee_id,
        auth_id: Some(auth_id),
        account_id: invite.account_id,
        ..invitee
    };

    // Update invite.
    let invite = Invite {
        id: invite.id,
        status: InviteStatus::Accepted,
        ..invite
    };

    db.persons().update(&invitee)?;
    db.invites().update(&invite)?;

    Ok(())
}
