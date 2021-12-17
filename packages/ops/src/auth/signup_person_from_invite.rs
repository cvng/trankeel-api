use crate::error::Result;
use async_graphql::InputObject;
use crate::Command;
use crate::error::Error;
use trankeel_data::Account;
use trankeel_data::AccountId;
use trankeel_data::AccountStatus;
use trankeel_data::AuthId;
use trankeel_data::Invite;
use trankeel_data::InviteReason;
use trankeel_data::InviteStatus;
use trankeel_data::InviteToken;
use trankeel_data::Person;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct SignupUserFromInviteInput {
    pub auth_id: AuthId,
    pub invite_token: InviteToken,
}

pub struct SignupUserFromInvitePayload {
    pub invite: Invite,
    pub invitee: Person,
    pub account: Account,
}

pub struct SignupUserFromInvite {
    invite: Invite,
    invitee: Person,
}

impl SignupUserFromInvite {
    pub fn new(invite: &Invite, invitee: &Person) -> Self {
        Self {
            invite: invite.clone(),
            invitee: invitee.clone(),
        }
    }
}

impl Command for SignupUserFromInvite {
    type Input = SignupUserFromInviteInput;
    type Payload = SignupUserFromInvitePayload;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self { invite, invitee } = self;

        if invite.reason != InviteReason::CandidacyAccepted {
            return Err(Error::msg("unknown invite reason"));
        }

        // Create account.
        let account = Account {
            id: AccountId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            plan_id: None,
            status: AccountStatus::Active, // Active account by default.
            stripe_customer_id: None,
            stripe_subscription_id: None,
            trial_end: None,
        };

        // Attach user with account.
        let invitee = Person {
            id: invite.invitee_id,
            auth_id: Some(input.auth_id),
            account_id: account.id,
            ..invitee
        };

        // Update invite.
        let invite = Invite {
            id: invite.id,
            status: InviteStatus::Accepted,
            ..invite
        };

        Ok(Self::Payload {
            invite,
            invitee,
            account,
        })
    }
}
