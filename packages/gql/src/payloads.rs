use crate::objects::Account;
use crate::objects::Advertisement;
use crate::objects::Candidacy;
use crate::objects::File;
use crate::objects::Lease;
use crate::objects::Lender;
use crate::objects::Message;
use crate::objects::Person;
use crate::objects::Property;
use crate::objects::Step;
use crate::objects::Tenant;
use async_graphql::SimpleObject;
use trankeel::DiscussionId;
use trankeel::LeaseId;
use trankeel::PropertyId;
use trankeel::TenantId;

#[derive(SimpleObject)]
pub struct CreateUserWithAccountPayload {
    user: Person,
    lender: Lender,
    account: Account,
    subscription: Option<trankeel::Subscription>,
}

impl From<trankeel::CreateUserWithAccountPayload> for CreateUserWithAccountPayload {
    fn from(item: trankeel::CreateUserWithAccountPayload) -> Self {
        Self {
            user: item.user.into(),
            lender: item.lender.into(),
            account: item.account.into(),
            subscription: None,
        }
    }
}

#[derive(SimpleObject)]
pub struct SignupUserFromInvitePayload {
    user: Person,
}

impl From<trankeel::Person> for SignupUserFromInvitePayload {
    fn from(item: trankeel::Person) -> Self {
        Self { user: item.into() }
    }
}

#[derive(SimpleObject)]
pub struct CompleteStepPayload {
    step: Step,
}

impl From<trankeel::Step> for CompleteStepPayload {
    fn from(item: trankeel::Step) -> Self {
        Self { step: item.into() }
    }
}

#[derive(SimpleObject)]
pub struct CreateNoticesPayload {
    notices: Vec<File>,
}

impl From<Vec<trankeel::Notice>> for CreateNoticesPayload {
    fn from(item: Vec<trankeel::Notice>) -> Self {
        Self {
            notices: item.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(SimpleObject)]
pub struct CreateReceiptsPayload {
    receipts: Vec<File>,
}

impl From<Vec<trankeel::Receipt>> for CreateReceiptsPayload {
    fn from(item: Vec<trankeel::Receipt>) -> Self {
        Self {
            receipts: item.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(SimpleObject)]
pub struct DeleteDiscussionPayload {
    id: DiscussionId,
}

impl From<trankeel::DiscussionId> for DeleteDiscussionPayload {
    fn from(item: trankeel::DiscussionId) -> Self {
        Self { id: item }
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

impl From<trankeel::Tenant> for UpdateTenantPayload {
    fn from(item: trankeel::Tenant) -> Self {
        Self {
            tenant: item.into(),
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
    message: Message,
}

impl From<trankeel::Message> for PushMessagePayload {
    fn from(item: trankeel::Message) -> Self {
        Self {
            message: item.into(),
        }
    }
}

#[derive(SimpleObject)]
pub struct DeleteTenantPayload {
    id: TenantId,
}

impl From<trankeel::TenantId> for DeleteTenantPayload {
    fn from(item: trankeel::TenantId) -> Self {
        Self { id: item }
    }
}

#[derive(SimpleObject)]
pub struct DeletePropertyPayload {
    id: PropertyId,
}

impl From<trankeel::PropertyId> for DeletePropertyPayload {
    fn from(item: trankeel::PropertyId) -> Self {
        Self { id: item }
    }
}

#[derive(SimpleObject)]
pub struct DeleteLeasePayload {
    id: LeaseId,
}

impl From<trankeel::LeaseId> for DeleteLeasePayload {
    fn from(item: trankeel::LeaseId) -> Self {
        Self { id: item }
    }
}

#[derive(SimpleObject)]
pub struct CreateCandidacyPayload {
    candidacy: Candidacy,
}

impl From<trankeel::Candidacy> for CreateCandidacyPayload {
    fn from(item: trankeel::Candidacy) -> Self {
        Self {
            candidacy: item.into(),
        }
    }
}

#[derive(SimpleObject)]
pub struct AcceptCandidacyPayload {
    candidacy: Candidacy,
}

impl From<trankeel::Candidacy> for AcceptCandidacyPayload {
    fn from(item: trankeel::Candidacy) -> Self {
        Self {
            candidacy: item.into(),
        }
    }
}

#[derive(SimpleObject)]
pub struct CreateLenderPayload {
    lender: Lender,
}

impl From<trankeel::Lender> for CreateLenderPayload {
    fn from(item: trankeel::Lender) -> Self {
        Self {
            lender: item.into(),
        }
    }
}

#[derive(SimpleObject)]
pub struct UpdateIndividualLenderPayload {
    lender: Lender,
}

impl From<trankeel::Lender> for UpdateIndividualLenderPayload {
    fn from(item: trankeel::Lender) -> Self {
        Self {
            lender: item.into(),
        }
    }
}

#[derive(SimpleObject)]
pub struct CreateLeasePayload {
    lease: Lease,
}

impl From<trankeel::Lease> for CreateLeasePayload {
    fn from(item: trankeel::Lease) -> Self {
        Self { lease: item.into() }
    }
}

#[derive(SimpleObject)]
pub struct UpdateLeasePayload {
    lease: Lease,
}

impl From<trankeel::Lease> for UpdateLeasePayload {
    fn from(item: trankeel::Lease) -> Self {
        Self { lease: item.into() }
    }
}
