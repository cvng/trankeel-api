use crate::context::Context;
use crate::database::Db;
use crate::error::Result;
use trankeel_data::Account;
use trankeel_data::AccountId;
use trankeel_data::AccountStatus;
use trankeel_data::Invite;
use trankeel_data::InviteStatus;
use trankeel_data::Person;
use trankeel_ops::event::InviteAccepted;

pub fn invite_accepted(ctx: &Context, event: InviteAccepted) -> Result<()> {
    let db = ctx.db();

    let InviteAccepted { invite_id, auth_id } = event;

    let invite = db.invites().by_id(&invite_id)?;
    let invitee = db.persons().by_id(&invite.invitee_id)?;

    // TODO: use invite.account_id and restrict access.
    let account = Account {
        id: AccountId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        plan_id: None,
        status: AccountStatus::Active,
        stripe_customer_id: None,
        stripe_subscription_id: None,
        trial_end: None,
    };

    // Attach user with account.
    let invitee = Person {
        id: invite.invitee_id,
        auth_id: Some(auth_id),
        account_id: account.id,
        ..invitee
    };

    // Update invite.
    let invite = Invite {
        id: invite.id,
        status: InviteStatus::Accepted,
        ..invite
    };

    db.accounts().create(&account)?;
    db.persons().update(&invitee)?;
    db.invites().update(&invite)?;

    Ok(())
}
