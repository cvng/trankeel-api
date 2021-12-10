use crate::objects::Account;
use crate::objects::Advertisement;
use crate::objects::Error;
use crate::objects::File;
use crate::objects::Lease;
use crate::objects::Lender;
use crate::objects::Message;
use crate::objects::Person;
use crate::objects::Property;
use crate::objects::Step;
use crate::objects::Tenant;
use trankeel::DiscussionId;

#[derive(SimpleObject)]
pub struct CompleteStepPayload {
    errors: Option<Vec<Error>>,
    step: Option<Step>,
}

impl From<trankeel::Result<trankeel::Step>> for CompleteStepPayload {
    fn from(item: trankeel::Result<trankeel::Step>) -> Self {
        match item {
            Ok(res) => Self {
                errors: None,
                step: Some(res.into()),
            },
            Err(err) => Self {
                errors: Some(vec![err.into()]),
                step: None,
            },
        }
    }
}

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
pub struct CreateTenantPayload {
    tenant: Tenant,
}

impl From<trankeel::Tenant> for CreateTenantPayload {
    fn from(item: trankeel::Tenant) -> Self {
        Self {
            tenant: item.into(),
        }
    }
}

#[derive(SimpleObject)]
pub struct UpdateTenantPayload {
    tenant: Tenant,
}

impl From<trankeel::UpdateTenantPayload> for UpdateTenantPayload {
    fn from(item: trankeel::UpdateTenantPayload) -> Self {
        Self {
            tenant: item.tenant.into(),
        }
    }
}

#[derive(SimpleObject)]
pub struct CreatePropertyPayload {
    property: Property,
}

impl From<trankeel::Property> for CreatePropertyPayload {
    fn from(item: trankeel::Property) -> Self {
        Self {
            property: item.into(),
        }
    }
}

#[derive(SimpleObject)]
pub struct UpdatePropertyPayload {
    property: Property,
}

impl From<trankeel::Property> for UpdatePropertyPayload {
    fn from(item: trankeel::Property) -> Self {
        Self {
            property: item.into(),
        }
    }
}

#[derive(SimpleObject)]
pub struct CreateAdvertisementPayload {
    advertisement: Advertisement,
}

impl From<trankeel::Advertisement> for CreateAdvertisementPayload {
    fn from(item: trankeel::Advertisement) -> Self {
        Self {
            advertisement: item.into(),
        }
    }
}

#[derive(SimpleObject)]
pub struct UpdateAdvertisementPayload {
    advertisement: Advertisement,
}

impl From<trankeel::Advertisement> for UpdateAdvertisementPayload {
    fn from(item: trankeel::Advertisement) -> Self {
        Self {
            advertisement: item.into(),
        }
    }
}

#[derive(SimpleObject)]
pub struct PushMessagePayload {
    errors: Option<Vec<Error>>,
    message: Option<Message>,
}

impl From<trankeel::PushMessagePayload> for PushMessagePayload {
    fn from(item: trankeel::PushMessagePayload) -> Self {
        Self {
            errors: Some(vec![]),
            message: Some(item.message.into()),
        }
    }
}

#[derive(SimpleObject)]
pub struct AddExistingLeasePayload {
    lease: Lease,
}

impl From<trankeel::Lease> for AddExistingLeasePayload {
    fn from(item: trankeel::Lease) -> Self {
        Self { lease: item.into() }
    }
}

#[derive(SimpleObject)]
pub struct CreateUserWithAccountPayload {
    account: Account,
    user: Person,
    lender: Lender,
    subscription: Option<trankeel::Subscription>,
}

impl From<trankeel::CreateUserWithAccountPayload> for CreateUserWithAccountPayload {
    fn from(item: trankeel::CreateUserWithAccountPayload) -> Self {
        Self {
            account: item.account.into(),
            user: item.user.into(),
            lender: item.lender.into(),
            subscription: item.subscription,
        }
    }
}
