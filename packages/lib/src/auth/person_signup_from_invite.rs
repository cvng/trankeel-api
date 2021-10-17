use crate::error::Result;
use async_graphql::InputObject;
use trankeel_core::database::Db;
use trankeel_core::error::Error;
use trankeel_data::Account;
use trankeel_data::AccountId;
use trankeel_data::AccountStatus;
use trankeel_data::AuthId;
use trankeel_data::InviteData;
use trankeel_data::InviteReason;
use trankeel_data::InviteStatus;
use trankeel_data::InviteToken;
use trankeel_data::Person;
use trankeel_data::PersonData;
use validator::Validate;

#[derive(Clone, InputObject, Validate)]
pub struct SignupUserFromInviteInput {
    pub auth_id: AuthId,
    pub invite_token: InviteToken,
}

pub async fn signup_user_from_invite(
    db: &impl Db,
    input: SignupUserFromInviteInput,
) -> Result<Person> {
    input.validate()?;

    let invite = db.invites().by_token(&input.invite_token)?;

    if invite.reason != InviteReason::CandidacyAccepted {
        return Err(Error::msg("unknown reason"));
    }

    // Create account.
    let account = db.accounts().create(Account {
        id: AccountId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        plan_id: None,
        status: AccountStatus::Active, // Active account by default.
        stripe_customer_id: None,
        stripe_subscription_id: None,
        trial_end: None,
    })?;

    // Attach user with account.
    let user = db.persons().update(PersonData {
        id: invite.invitee_id,
        auth_id: Some(input.auth_id),
        account_id: Some(account.id),
        ..Default::default()
    })?;

    // Update invite.
    db.invites().update(InviteData {
        id: invite.id,
        status: Some(InviteStatus::Accepted),
        ..Default::default()
    })?;

    Ok(user)
}
