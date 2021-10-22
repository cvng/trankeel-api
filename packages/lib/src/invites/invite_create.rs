use crate::error::Result;
use trankeel_core::database::Db;
use trankeel_data::Invite;
use trankeel_data::InviteId;
use trankeel_data::InviteReason;
use trankeel_data::InviteStatus;
use trankeel_data::InviteToken;
use trankeel_data::PersonId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateInviteInput {
    #[graphql(name = "id")]
    pub invitee_id: PersonId,
    pub reason: InviteReason,
}

pub fn create_invite(db: &impl Db, input: CreateInviteInput) -> Result<Invite> {
    input.validate()?;

    let invitee = db.persons().by_id(&input.invitee_id)?;

    let id = InviteId::new();
    let token = InviteToken::new(id.to_string());

    db.invites().create(Invite {
        id,
        created_at: Default::default(),
        updated_at: Default::default(),
        account_id: None,
        invitee_id: invitee.id,
        token,
        status: InviteStatus::default(),
        reason: input.reason,
    })
}
