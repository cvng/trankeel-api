use super::push_message;
use crate::messaging::PushMessageState;
use crate::PushMessageInput;
use crate::Result;
use chrono::Utc;
use trankeel_data::Account;
use trankeel_data::Discussion;
use trankeel_data::DiscussionId;
use trankeel_data::Message;
use trankeel_data::PersonId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateDiscussionInput {
    pub initiator_id: PersonId,
    pub recipient_id: PersonId,
    pub message: Option<String>,
}

pub struct CreateDiscussionState {
    pub account: Account,
}

pub struct CreateDiscussionPayload {
    pub discussion: Discussion,
    pub message: Option<Message>,
}

pub fn create_discussion(
    state: CreateDiscussionState,
    input: CreateDiscussionInput,
) -> Result<CreateDiscussionPayload> {
    input.validate()?;

    let discussion = Discussion {
        id: DiscussionId::new(),
        created_at: Default::default(),
        updated_at: Some(Utc::now().into()), // Touch updated_at.
        account_id: state.account.id,
        initiator_id: input.initiator_id,
        status: Default::default(),
    };

    let message = if let Some(message) = input.message {
        let message = push_message(
            PushMessageState {
                discussion: discussion.clone(),
            },
            PushMessageInput {
                discussion_id: discussion.id,
                sender_id: input.initiator_id,
                message,
            },
        )?
        .message;

        Some(message)
    } else {
        None
    };

    Ok(CreateDiscussionPayload {
        discussion,
        message,
    })
}
