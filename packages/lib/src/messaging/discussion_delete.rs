use crate::Result;
use piteo_core::database::Db;
use piteo_data::AuthId;
use piteo_data::DiscussionId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct DeleteDiscussionInput {
    pub id: DiscussionId,
}

pub fn delete_discussion(
    db: &impl Db,
    _auth_id: &AuthId,
    input: DeleteDiscussionInput,
) -> Result<DiscussionId> {
    input.validate()?;

    db.discussions().delete(input.id)?;

    Ok(input.id)
}
