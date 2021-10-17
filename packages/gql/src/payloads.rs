use crate::objects::Error;
use crate::objects::File;
use crate::objects::Message;
use trankeel::DiscussionId;

#[derive(SimpleObject)]
pub struct CreateNoticesPayload {
    errors: Option<Vec<Error>>,
    notices: Option<Vec<File>>,
}

impl From<trankeel::Result<Vec<trankeel::Notice>>> for CreateNoticesPayload {
    fn from(item: trankeel::Result<Vec<trankeel::Notice>>) -> Self {
        match item {
            Ok(res) => Self {
                errors: None,
                notices: Some(res.into_iter().map(Into::into).collect()),
            },
            Err(err) => Self {
                errors: Some(vec![err.into()]),
                notices: None,
            },
        }
    }
}

#[derive(SimpleObject)]
#[graphql(name = "RentReceiptPayload")]
pub struct CreateReceiptsPayload {
    errors: Option<Vec<Error>>,
    receipts: Option<Vec<File>>,
}

impl From<trankeel::Result<Vec<trankeel::Receipt>>> for CreateReceiptsPayload {
    fn from(item: trankeel::Result<Vec<trankeel::Receipt>>) -> Self {
        match item {
            Ok(res) => Self {
                errors: None,
                receipts: Some(res.into_iter().map(Into::into).collect()),
            },
            Err(err) => Self {
                errors: Some(vec![err.into()]),
                receipts: None,
            },
        }
    }
}

#[derive(SimpleObject)]
pub struct DeleteDiscussionPayload {
    errors: Option<Vec<Error>>,
    id: Option<DiscussionId>,
}

impl From<trankeel::Result<trankeel::DiscussionId>> for DeleteDiscussionPayload {
    fn from(item: trankeel::Result<trankeel::DiscussionId>) -> Self {
        match item {
            Ok(res) => Self {
                errors: None,
                id: Some(res),
            },
            Err(err) => Self {
                errors: Some(vec![err.into()]),
                id: None,
            },
        }
    }
}

#[derive(SimpleObject)]
pub struct PushMessagePayload {
    errors: Option<Vec<Error>>,
    message: Option<Message>,
}

impl From<trankeel::Result<trankeel::Message>> for PushMessagePayload {
    fn from(item: trankeel::Result<trankeel::Message>) -> Self {
        match item {
            Ok(res) => Self {
                errors: None,
                message: Some(res.into()),
            },
            Err(err) => Self {
                errors: Some(vec![err.into()]),
                message: None,
            },
        }
    }
}
