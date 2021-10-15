use super::push_message;
use crate::PushMessageInput;
use crate::Result;
use piteo_core::database::Db;
use piteo_data::Discussion;
use piteo_data::DiscussionId;
use piteo_data::PersonId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateDiscussionInput {
    pub initiator_id: PersonId,
    pub recipient_id: PersonId,
    pub message: Option<String>,
}

pub fn create_discussion_unauthenticated(
    db: &impl Db,
    input: CreateDiscussionInput,
) -> Result<Discussion> {
    input.validate()?;

    let account = db.accounts().by_person_id(&input.recipient_id)?;

    let discussion = db.discussions().create(Discussion {
        id: DiscussionId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        account_id: account.id,
        initiator_id: input.initiator_id,
        status: Default::default(),
    })?;

    db.discussions().touch(discussion.id)?; // Touch updated_at.

    if let Some(message) = input.message {
        push_message(
            db,
            PushMessageInput {
                discussion_id: discussion.id,
                sender_id: input.initiator_id,
                message,
            },
        )?;
    }

    Ok(discussion)
}
