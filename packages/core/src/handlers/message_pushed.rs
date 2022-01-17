use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Handler;
use crate::error::Result;
use chrono::Utc;
use trankeel_data::Discussion;
use trankeel_data::Message;
use trankeel_ops::event::MessagePushed;

pub struct MessagePushedPayload {
    pub message: Message,
    pub discussion: Discussion,
}

pub struct MessagePushedHandler {
    pub discussion: Discussion,
}

impl MessagePushedHandler {
    pub fn new(discussion: &Discussion) -> Self {
        Self {
            discussion: discussion.clone(),
        }
    }
}

impl Handler for MessagePushedHandler {
    type Event = MessagePushed;
    type Payload = MessagePushedPayload;

    fn run(self, event: Self::Event) -> Result<Self::Payload> {
        let Self { discussion } = self;

        let message = Message { ..event.message };

        let discussion = Discussion {
            updated_at: Some(Utc::now().into()), // Touch updated_at.
            ..discussion
        };

        Ok(Self::Payload {
            message,
            discussion,
        })
    }
}

pub fn message_pushed(ctx: &Context, event: MessagePushed) -> Result<()> {
    let db = ctx.db();

    let discussion = db.discussions().by_id(&event.message.discussion_id)?;

    MessagePushedHandler::new(&discussion)
        .run(event)
        .and_then(|payload| {
            db.messages().create(&payload.message)?;
            db.discussions().update(&payload.discussion)?;
            Ok(())
        })
}
