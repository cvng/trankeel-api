use crate::error::Result;
use piteo_core::database::Db;
use piteo_data::Invite;
use piteo_data::InviteId;
use piteo_data::InviteReason;
use piteo_data::InviteStatus;
use piteo_data::InviteToken;
use piteo_data::PersonId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateInviteInput {
    pub id: PersonId,
    pub reason: InviteReason,
}

pub fn create_invite(db: &impl Db, input: CreateInviteInput) -> Result<Invite> {
    input.validate()?;

    let invitee = db.persons().by_id(&input.id)?;

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
