use super::push_message;
use super::PushMessagePayload;
use crate::messaging::message_push;
use crate::PushMessageInput;
use crate::Result;
use chrono::Utc;
use message_push::PushMessageState;
use trankeel_data::Account;
use trankeel_data::DateTime;
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
    pub messages: Vec<Message>,
}

pub fn create_discussion(
    input: CreateDiscussionInput,
    state: CreateDiscussionState,
) -> Result<CreateDiscussionPayload> {
    input.validate()?;

    let discussion = Discussion {
        id: DiscussionId::new(),
        created_at: Default::default(),
        updated_at: Some(DateTime(Utc::now())),
        account_id: state.account.id,
        initiator_id: input.initiator_id,
        status: Default::default(),
    };

    if let Some(message) = input.message {
        let PushMessagePayload {
            discussion,
            message,
        } = push_message(
            PushMessageInput {
                discussion_id: discussion.id,
                sender_id: input.initiator_id,
                message,
            },
            PushMessageState { discussion },
        )?;

        Ok(CreateDiscussionPayload {
            discussion,
            messages: vec![message],
        })
    } else {
        Ok(CreateDiscussionPayload {
            discussion,
            messages: vec![],
        })
    }
}
