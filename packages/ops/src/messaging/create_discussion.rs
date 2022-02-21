use super::PushMessage;
use super::PushMessageInput;
use crate::command::Command;
use crate::error::Result;
use crate::event::DiscussionCreated;
use crate::event::Event;
use async_graphql::InputObject;
use chrono::Utc;
use trankeel_data::Account;
use trankeel_data::Discussion;
use trankeel_data::DiscussionId;
use trankeel_data::MessageId;
use trankeel_data::PersonId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateDiscussionInput {
    pub initiator_id: PersonId,
    pub recipient_id: PersonId,
    pub message: Option<String>,
}

pub struct CreateDiscussion {
    account: Account,
}

impl CreateDiscussion {
    pub fn new(account: &Account) -> Self {
        Self {
            account: account.clone(),
        }
    }
}

impl Command for CreateDiscussion {
    type Input = CreateDiscussionInput;

    fn run(self, input: Self::Input) -> Result<Vec<Event>> {
        input.validate()?;

        let Self { account } = self;

        let discussion = Discussion {
            id: DiscussionId::new(),
            created_at: Default::default(),
            updated_at: Some(Utc::now().into()), // Touch updated_at.
            account_id: account.id,
            initiator_id: input.initiator_id,
            status: Default::default(),
        };

        let (discussion, message) = if let Some(message) = input.message {
            let message = PushMessage::new(MessageId::new())
                .run(PushMessageInput {
                    discussion_id: discussion.id,
                    sender_id: input.initiator_id,
                    message,
                })?
                .into_iter()
                .find_map(|event| match event {
                    Event::MessagePushed(event) => Some(event.message),
                    _ => None,
                })
                .unwrap();

            (discussion, Some(message))
        } else {
            (discussion, None)
        };

        Ok(vec![DiscussionCreated {
            discussion,
            message,
        }
        .into()])
    }
}
