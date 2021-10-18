use crate::messaging::message_push;
use crate::Command;
use crate::PushMessageInput;
use crate::Result;
use chrono::Utc;
use message_push::PushMessageCommand;
use trankeel_data::Account;
use trankeel_data::DateTime;
use trankeel_data::Discussion;
use trankeel_data::DiscussionId;
use trankeel_data::DiscussionWithMessages;
use trankeel_data::PersonId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateDiscussionInput {
    pub initiator_id: PersonId,
    pub recipient_id: PersonId,
    pub message: Option<String>,
}

pub struct State {
    pub account: Account,
}

pub struct Output {
    pub discussion: DiscussionWithMessages,
}

pub(crate) struct CreateDiscussionCommand;

impl Command for CreateDiscussionCommand {
    type Input = CreateDiscussionInput;
    type State = State;
    type Output = Output;

    fn run(state: Self::State, input: Self::Input) -> Result<Self::Output> {
        input.validate()?;

        let account = state.account;

        let mut discussion = Discussion {
            id: DiscussionId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            account_id: account.id,
            initiator_id: input.initiator_id,
            status: Default::default(),
        };

        discussion.updated_at = Some(DateTime(Utc::now())); // Touch updated_at.

        let mut messages = vec![];

        if let Some(message) = input.message {
            messages.push(
                PushMessageCommand::run(
                    message_push::State {
                        discussion: discussion.clone(),
                    },
                    PushMessageInput {
                        discussion_id: discussion.id,
                        sender_id: input.initiator_id,
                        message,
                    },
                )?
                .message,
            );
        }

        Ok(Output {
            discussion: (discussion, messages),
        })
    }
}
