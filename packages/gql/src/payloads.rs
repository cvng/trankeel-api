use crate::objects::Error;
use crate::objects::File;
use crate::objects::Message;
use piteo::DiscussionId;

#[derive(SimpleObject)]
pub struct CreateNoticesPayload {
    errors: Option<Vec<Error>>,
    notices: Option<Vec<File>>,
}

impl From<piteo::Result<Vec<piteo::Notice>>> for CreateNoticesPayload {
    fn from(item: piteo::Result<Vec<piteo::Notice>>) -> Self {
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

impl From<piteo::Result<Vec<piteo::Receipt>>> for CreateReceiptsPayload {
    fn from(item: piteo::Result<Vec<piteo::Receipt>>) -> Self {
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

impl From<piteo::Result<piteo::DiscussionId>> for DeleteDiscussionPayload {
    fn from(item: piteo::Result<piteo::DiscussionId>) -> Self {
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

impl From<piteo::Result<piteo::Message>> for PushMessagePayload {
    fn from(item: piteo::Result<piteo::Message>) -> Self {
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
