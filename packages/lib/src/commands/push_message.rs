use crate::client::Context;
use crate::messaging;
use crate::messaging::PushMessagePayload;
use crate::messaging::PushMessageState;
use crate::Command;
use crate::PushMessageInput;
use crate::Result;
use trankeel_core::database::Db;

pub(crate) struct PushMessage<'a> {
    context: &'a Context,
}

impl<'a> PushMessage<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }
}

impl<'a> Command for PushMessage<'a> {
    type Input = PushMessageInput;
    type Payload = PushMessagePayload;

    fn run(&self, input: Self::Input) -> Result<Self::Payload> {
        let db = self.context.db();

        let state = PushMessageState {
            discussion: db.discussions().by_id(&input.discussion_id)?,
        };

        let payload = messaging::push_message(state, input)?;

        db.transaction(|| {
            db.messages().create(&payload.message)?;
            db.discussions().update(&payload.discussion)?;
            Ok(())
        })?;

        Ok(payload)
    }
}
