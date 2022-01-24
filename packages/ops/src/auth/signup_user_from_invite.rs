use crate::error::DomainError;
use crate::error::Result;
use crate::event::AccountCreated;
use crate::event::Event;
use crate::event::InviteAccepted;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::Account;
use trankeel_data::AccountId;
use trankeel_data::AccountStatus;
use trankeel_data::AuthId;
use trankeel_data::Invite;
use trankeel_data::InviteReason;
use trankeel_data::InviteToken;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct SignupUserFromInviteInput {
    pub auth_id: AuthId,
    pub invite_token: InviteToken,
}

pub struct SignupUserFromInvite {
    invite: Invite,
}

impl SignupUserFromInvite {
    pub fn new(invite: &Invite) -> Self {
        Self {
            invite: invite.clone(),
        }
    }
}

impl Command for SignupUserFromInvite {
    type Input = SignupUserFromInviteInput;
    type Payload = Vec<Event>;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self { invite } = self;

        if invite.reason != InviteReason::CandidacyAccepted {
            return Err(DomainError::InviteReasonUnimplemented(invite.reason).into());
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

        Ok(vec![
            AccountCreated { account }.into(),
            InviteAccepted {
                invite_id: invite.id,
                auth_id: input.auth_id,
            }
            .into(),
        ])
    }
}
