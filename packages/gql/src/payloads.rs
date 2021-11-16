use crate::objects::Account;
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

impl From<trankeel::CreateTenantPayload> for CreateTenantPayload {
    fn from(item: trankeel::CreateTenantPayload) -> Self {
        Self {
            tenant: item.tenant.0.into(),
        }
    }
}

#[derive(SimpleObject)]
pub struct CreatePropertyPayload {
    property: Property,
}

impl From<trankeel::CreatePropertyPayload> for CreatePropertyPayload {
    fn from(item: trankeel::CreatePropertyPayload) -> Self {
        Self {
            property: item.property.into(),
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
    property: Property,
    tenants: Vec<Tenant>,
}

impl From<trankeel::AddExistingLeasePayload> for AddExistingLeasePayload {
    fn from(item: trankeel::AddExistingLeasePayload) -> Self {
        Self {
            lease: item.lease.into(),
            property: item.property.into(),
            tenants: item
                .tenants
                .into_iter()
                .map(|tenant| tenant.0.into())
                .collect(),
        }
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
