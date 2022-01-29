use crate::error::Result;
use crate::event::Event;
use crate::event::InviteCreated;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::AccountId;
use trankeel_data::Invite;
use trankeel_data::InviteId;
use trankeel_data::InviteReason;
use trankeel_data::InviteStatus;
use trankeel_data::InviteToken;
use trankeel_data::Person;
use trankeel_data::PersonId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateInviteInput {
    pub invitee_id: PersonId,
    pub account_id: AccountId,
    pub reason: InviteReason,
}

pub struct CreateInvite {
    invitee: Person,
}

impl CreateInvite {
    pub fn new(invitee: &Person) -> Self {
        Self {
            invitee: invitee.clone(),
        }
    }
}

impl Command for CreateInvite {
    type Input = CreateInviteInput;
    type Payload = Vec<Event>;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self { invitee } = self;

        let id = InviteId::new();
        let token = InviteToken::new(id.to_string());

        let invite = Invite {
            id,
            created_at: Default::default(),
            updated_at: Default::default(),
            account_id: input.account_id,
            invitee_id: invitee.id,
            token,
            status: InviteStatus::default(),
            reason: input.reason,
        };

        Ok(vec![InviteCreated { invite }.into()])
    }
}
