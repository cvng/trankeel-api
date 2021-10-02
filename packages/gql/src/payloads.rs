use crate::objects::Error;
use crate::objects::File;
use crate::objects::Message;
use async_graphql::ID;

#[derive(async_graphql::SimpleObject)]
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
                receipts: Some(map_into(res)),
            },
            Err(err) => Self {
                errors: Some(vec![err.into()]),
                receipts: None,
            },
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct CreateNoticesPayload {
    errors: Option<Vec<Error>>,
    notices: Option<Vec<File>>,
}

impl From<piteo::Result<Vec<piteo::PaymentNotice>>> for CreateNoticesPayload {
    fn from(item: piteo::Result<Vec<piteo::PaymentNotice>>) -> Self {
        match item {
            Ok(res) => Self {
                errors: None,
                notices: Some(map_into(res)),
            },
            Err(err) => Self {
                errors: Some(vec![err.into()]),
                notices: None,
            },
        }
    }
}

#[derive(async_graphql::SimpleObject)]
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

#[derive(async_graphql::SimpleObject)]
pub struct DeleteDiscussionPayload {
    errors: Option<Vec<Error>>,
    id: Option<ID>,
}

impl From<piteo::Result<piteo::DiscussionId>> for DeleteDiscussionPayload {
    fn from(item: piteo::Result<piteo::DiscussionId>) -> Self {
        match item {
            Ok(res) => Self {
                errors: None,
                id: Some(res.into()),
            },
            Err(err) => Self {
                errors: Some(vec![err.into()]),
                id: None,
            },
        }
    }
}

fn map_into<T, U>(vec: Vec<T>) -> Vec<U>
where
    T: Clone,
    U: From<T>,
{
    vec.iter().map(|item| item.clone().into()).collect()
}
