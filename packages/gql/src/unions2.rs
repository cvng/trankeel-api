use crate::objects::Account;
use crate::objects::Advertisement;
use crate::objects::Candidacy;
use crate::objects::Company;
use crate::objects::Discussion;
use crate::objects::Lease;
use crate::objects::Lender;
use crate::objects::Message;
use crate::objects::Notice;
use crate::objects::Payment;
use crate::objects::Person;
use crate::objects::Property;
use crate::objects::Receipt;
use crate::objects::Rent;
use crate::objects::Step;
use crate::objects::Tenant;
use crate::objects::WarrantWithIdentity;
use crate::objects::Workflow;
use crate::unions::LegalIdentity;
use crate::unions::Workflowable;
use async_graphql::SimpleObject;
use async_graphql::Union;
use trankeel::event;
use trankeel::AccountId;
use trankeel::AuthId;
use trankeel::CandidacyId;
use trankeel::DiscussionId;
use trankeel::Document;
use trankeel::Email;
use trankeel::Invite;
use trankeel::InviteId;
use trankeel::LeaseId;
use trankeel::PropertyId;
use trankeel::RentId;
use trankeel::StepId;
use trankeel::TenantId;

#[allow(clippy::large_enum_variant)]
#[derive(Union)]
pub enum Listenable {
    AccountCreated(AccountCreated),
    AdvertisementCreated(AdvertisementCreated),
    AdvertisementUpdated(AdvertisementUpdated),
    CandidacyAccepted(CandidacyAccepted),
    CandidacyCreated(CandidacyCreated),
    CandidacyRejected(CandidacyRejected),
    CompanyCreated(CompanyCreated),
    DiscussionCreated(DiscussionCreated),
    DiscussionDeleted(DiscussionDeleted),
    DocumentGenerated(DocumentGenerated),
    InviteAccepted(InviteAccepted),
    InviteCreated(InviteCreated),
    LeaseAffected(LeaseAffected),
    LeaseCreated(LeaseCreated),
    LeaseDeleted(LeaseDeleted),
    LeaseFileRequested(LeaseFileRequested),
    LeaseUpdated(LeaseUpdated),
    LenderCreated(LenderCreated),
    LenderUpdated(LenderUpdated),
    MessagePushed(MessagePushed),
    NoticeCreated(NoticeCreated),
    PaymentCreated(PaymentCreated),
    PersonCreated(PersonCreated),
    PropertyCreated(PropertyCreated),
    PropertyDeleted(PropertyDeleted),
    PropertyUpdated(PropertyUpdated),
    ReceiptCreated(ReceiptCreated),
    ReceiptSent(ReceiptSent),
    StepCompleted(StepCompleted),
    StepCreated(StepCreated),
    SubscriptionRequested(SubscriptionRequested),
    TenantCreated(TenantCreated),
    TenantDeleted(TenantDeleted),
    TenantUpdated(TenantUpdated),
    WarrantCreated(WarrantCreated),
    WorkflowCreated(WorkflowCreated),
}

impl From<event::Event> for Listenable {
    fn from(item: event::Event) -> Self {
        match item {
            event::Event::AccountCreated(evt) => Self::AccountCreated(evt.into()),
            event::Event::AdvertisementCreated(evt) => Self::AdvertisementCreated(evt.into()),
            event::Event::AdvertisementUpdated(evt) => Self::AdvertisementUpdated(evt.into()),
            event::Event::CandidacyAccepted(evt) => Self::CandidacyAccepted(evt.into()),
            event::Event::CandidacyCreated(evt) => Self::CandidacyCreated(evt.into()),
            event::Event::CandidacyRejected(evt) => Self::CandidacyRejected(evt.into()),
            event::Event::CompanyCreated(evt) => Self::CompanyCreated(evt.into()),
            event::Event::DiscussionCreated(evt) => Self::DiscussionCreated(evt.into()),
            event::Event::DiscussionDeleted(evt) => Self::DiscussionDeleted(evt.into()),
            event::Event::DocumentGenerated(evt) => Self::DocumentGenerated(evt.into()),
            event::Event::InviteAccepted(evt) => Self::InviteAccepted(evt.into()),
            event::Event::InviteCreated(evt) => Self::InviteCreated(evt.into()),
            event::Event::LeaseAffected(evt) => Self::LeaseAffected(evt.into()),
            event::Event::LeaseCreated(evt) => Self::LeaseCreated(evt.into()),
            event::Event::LeaseDeleted(evt) => Self::LeaseDeleted(evt.into()),
            event::Event::LeaseFileRequested(evt) => Self::LeaseFileRequested(evt.into()),
            event::Event::LeaseUpdated(evt) => Self::LeaseUpdated(evt.into()),
            event::Event::LenderCreated(evt) => Self::LenderCreated(evt.into()),
            event::Event::LenderUpdated(evt) => Self::LenderUpdated(evt.into()),
            event::Event::MessagePushed(evt) => Self::MessagePushed(evt.into()),
            event::Event::NoticeCreated(evt) => Self::NoticeCreated(evt.into()),
            event::Event::PaymentCreated(evt) => Self::PaymentCreated(evt.into()),
            event::Event::PersonCreated(evt) => Self::PersonCreated(evt.into()),
            event::Event::PropertyCreated(evt) => Self::PropertyCreated(evt.into()),
            event::Event::PropertyDeleted(evt) => Self::PropertyDeleted(evt.into()),
            event::Event::PropertyUpdated(evt) => Self::PropertyUpdated(evt.into()),
            event::Event::ReceiptCreated(evt) => Self::ReceiptCreated(evt.into()),
            event::Event::ReceiptSent(evt) => Self::ReceiptSent(evt.into()),
            event::Event::StepCompleted(evt) => Self::StepCompleted(evt.into()),
            event::Event::StepCreated(evt) => Self::StepCreated(evt.into()),
            event::Event::SubscriptionRequested(evt) => Self::SubscriptionRequested(evt.into()),
            event::Event::TenantCreated(evt) => Self::TenantCreated(evt.into()),
            event::Event::TenantDeleted(evt) => Self::TenantDeleted(evt.into()),
            event::Event::TenantUpdated(evt) => Self::TenantUpdated(evt.into()),
            event::Event::WarrantCreated(evt) => Self::WarrantCreated(evt.into()),
            event::Event::WorkflowCreated(evt) => Self::WorkflowCreated(evt.into()),
        }
    }
}

#[derive(SimpleObject)]
pub struct AccountCreated {
    pub account: Account,
}

#[derive(SimpleObject)]
pub struct AdvertisementCreated {
    pub advertisement: Advertisement,
}

#[derive(SimpleObject)]
pub struct AdvertisementUpdated {
    pub advertisement: Advertisement,
}

#[derive(SimpleObject)]
pub struct CandidacyAccepted {
    pub candidacy_id: CandidacyId,
}

#[derive(SimpleObject)]
pub struct CandidacyCreated {
    pub candidacy: Candidacy,
}

#[derive(SimpleObject)]
pub struct CandidacyRejected {
    pub candidacy_id: CandidacyId,
}

#[derive(SimpleObject)]
pub struct CompanyCreated {
    pub company: Company,
}

#[derive(SimpleObject)]
pub struct DiscussionCreated {
    pub discussion: Discussion,
    pub message: Option<Message>,
}

#[derive(SimpleObject)]
pub struct DiscussionDeleted {
    pub discussion_id: DiscussionId,
}

#[derive(SimpleObject)]
pub struct DocumentGenerated {
    pub document: Document,
}

#[derive(SimpleObject)]
pub struct InviteAccepted {
    pub invite_id: InviteId,
    pub auth_id: AuthId,
}

#[derive(SimpleObject)]
pub struct InviteCreated {
    pub invite: Invite,
}

#[derive(SimpleObject)]
pub struct LeaseAffected {
    pub lease_id: LeaseId,
    pub tenant_id: TenantId,
}

#[derive(SimpleObject)]
pub struct LeaseCreated {
    pub lease: Lease,
    pub rents: Vec<Rent>,
}

#[derive(SimpleObject)]
pub struct LeaseDeleted {
    pub lease_id: LeaseId,
}

#[derive(SimpleObject)]
pub struct LeaseFileRequested {
    pub lease_id: LeaseId,
}

#[derive(SimpleObject)]
pub struct LeaseUpdated {
    pub lease: Lease,
}

#[derive(SimpleObject)]
pub struct LenderCreated {
    pub lender: Lender,
}

#[derive(SimpleObject)]
pub struct LenderUpdated {
    pub lender: Lender,
    pub identity: LegalIdentity,
}

#[derive(SimpleObject)]
pub struct MessagePushed {
    pub message: Message,
}

#[derive(SimpleObject)]
pub struct NoticeCreated {
    pub notice: Notice,
    pub rent: Rent,
}

#[derive(SimpleObject)]
pub struct PaymentCreated {
    pub payment: Payment,
}

#[derive(SimpleObject)]
pub struct PersonCreated {
    pub person: Person,
}

#[derive(SimpleObject)]
pub struct PropertyCreated {
    pub property: Property,
}

#[derive(SimpleObject)]
pub struct PropertyDeleted {
    pub property_id: PropertyId,
}

#[derive(SimpleObject)]
pub struct PropertyUpdated {
    pub property: Property,
}

#[derive(SimpleObject)]
pub struct ReceiptCreated {
    pub receipt: Receipt,
    pub rent: Rent,
    pub payment: Payment,
}

#[derive(SimpleObject)]
pub struct ReceiptSent {
    pub rent_id: RentId,
}

#[derive(SimpleObject)]
pub struct StepCompletedRequirement {
    pub name: String,
    pub value: String,
}

impl From<event::StepCompletedRequirement> for StepCompletedRequirement {
    fn from(item: event::StepCompletedRequirement) -> Self {
        Self {
            name: item.name,
            value: item.value,
        }
    }
}

#[derive(SimpleObject)]
pub struct StepCompleted {
    pub step_id: StepId,
    pub requirements: Option<Vec<StepCompletedRequirement>>,
}

#[derive(SimpleObject)]
pub struct StepCreated {
    pub step: Step,
}

#[derive(SimpleObject)]
pub struct SubscriptionRequested {
    pub account_id: AccountId,
    pub email: Email,
}

#[derive(SimpleObject)]
pub struct TenantCreated {
    pub tenant: Tenant,
    pub identity: Option<Person>,
    pub warrants: Option<Vec<WarrantWithIdentity>>,
    pub discussion: Option<Discussion>,
}

#[derive(SimpleObject)]
pub struct TenantDeleted {
    pub tenant_id: TenantId,
}

#[derive(SimpleObject)]
pub struct TenantUpdated {
    pub tenant: Tenant,
}

#[derive(SimpleObject)]
pub struct WarrantCreated {
    pub warrant: WarrantWithIdentity,
}

#[derive(SimpleObject)]
pub struct WorkflowCreated {
    pub workflow: Workflow,
    pub workflowable: Workflowable,
}

//

impl From<event::AccountCreated> for AccountCreated {
    fn from(item: event::AccountCreated) -> Self {
        Self {
            account: item.account.into(),
        }
    }
}

impl From<event::AdvertisementCreated> for AdvertisementCreated {
    fn from(item: event::AdvertisementCreated) -> Self {
        Self {
            advertisement: item.advertisement.into(),
        }
    }
}

impl From<event::AdvertisementUpdated> for AdvertisementUpdated {
    fn from(item: event::AdvertisementUpdated) -> Self {
        Self {
            advertisement: item.advertisement.into(),
        }
    }
}

impl From<event::CandidacyAccepted> for CandidacyAccepted {
    fn from(item: event::CandidacyAccepted) -> Self {
        Self {
            candidacy_id: item.candidacy_id,
        }
    }
}

impl From<event::CandidacyCreated> for CandidacyCreated {
    fn from(item: event::CandidacyCreated) -> Self {
        Self {
            candidacy: item.candidacy.into(),
        }
    }
}

impl From<event::CandidacyRejected> for CandidacyRejected {
    fn from(item: event::CandidacyRejected) -> Self {
        Self {
            candidacy_id: item.candidacy_id,
        }
    }
}

impl From<event::CompanyCreated> for CompanyCreated {
    fn from(item: event::CompanyCreated) -> Self {
        Self {
            company: item.company.into(),
        }
    }
}

impl From<event::DiscussionCreated> for DiscussionCreated {
    fn from(item: event::DiscussionCreated) -> Self {
        Self {
            discussion: item.discussion.into(),
            message: item.message.map(Into::into),
        }
    }
}

impl From<event::DiscussionDeleted> for DiscussionDeleted {
    fn from(item: event::DiscussionDeleted) -> Self {
        Self {
            discussion_id: item.discussion_id,
        }
    }
}

impl From<event::DocumentGenerated> for DocumentGenerated {
    fn from(item: event::DocumentGenerated) -> Self {
        Self {
            document: item.document,
        }
    }
}

impl From<event::InviteAccepted> for InviteAccepted {
    fn from(item: event::InviteAccepted) -> Self {
        Self {
            invite_id: item.invite_id,
            auth_id: item.auth_id,
        }
    }
}

impl From<event::InviteCreated> for InviteCreated {
    fn from(item: event::InviteCreated) -> Self {
        Self {
            invite: item.invite,
        }
    }
}

impl From<event::LeaseAffected> for LeaseAffected {
    fn from(item: event::LeaseAffected) -> Self {
        Self {
            lease_id: item.lease_id,
            tenant_id: item.tenant_id,
        }
    }
}

impl From<event::LeaseCreated> for LeaseCreated {
    fn from(item: event::LeaseCreated) -> Self {
        Self {
            lease: item.lease.into(),
            rents: item.rents.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<event::LeaseDeleted> for LeaseDeleted {
    fn from(item: event::LeaseDeleted) -> Self {
        Self {
            lease_id: item.lease_id,
        }
    }
}

impl From<event::LeaseFileRequested> for LeaseFileRequested {
    fn from(item: event::LeaseFileRequested) -> Self {
        Self {
            lease_id: item.lease_id,
        }
    }
}

impl From<event::LeaseUpdated> for LeaseUpdated {
    fn from(item: event::LeaseUpdated) -> Self {
        Self {
            lease: item.lease.into(),
        }
    }
}

impl From<event::LenderCreated> for LenderCreated {
    fn from(item: event::LenderCreated) -> Self {
        Self {
            lender: item.lender.into(),
        }
    }
}

impl From<event::LenderUpdated> for LenderUpdated {
    fn from(item: event::LenderUpdated) -> Self {
        Self {
            lender: item.lender.into(),
            identity: item.identity.into(),
        }
    }
}

impl From<event::MessagePushed> for MessagePushed {
    fn from(item: event::MessagePushed) -> Self {
        Self {
            message: item.message.into(),
        }
    }
}

impl From<event::NoticeCreated> for NoticeCreated {
    fn from(item: event::NoticeCreated) -> Self {
        Self {
            notice: item.notice.into(),
            rent: item.rent.into(),
        }
    }
}

impl From<event::PaymentCreated> for PaymentCreated {
    fn from(item: event::PaymentCreated) -> Self {
        Self {
            payment: item.payment.into(),
        }
    }
}

impl From<event::PersonCreated> for PersonCreated {
    fn from(item: event::PersonCreated) -> Self {
        Self {
            person: item.person.into(),
        }
    }
}

impl From<event::PropertyCreated> for PropertyCreated {
    fn from(item: event::PropertyCreated) -> Self {
        Self {
            property: item.property.into(),
        }
    }
}

impl From<event::PropertyDeleted> for PropertyDeleted {
    fn from(item: event::PropertyDeleted) -> Self {
        Self {
            property_id: item.property_id,
        }
    }
}

impl From<event::PropertyUpdated> for PropertyUpdated {
    fn from(item: event::PropertyUpdated) -> Self {
        Self {
            property: item.property.into(),
        }
    }
}

impl From<event::ReceiptCreated> for ReceiptCreated {
    fn from(item: event::ReceiptCreated) -> Self {
        Self {
            receipt: item.receipt.into(),
            rent: item.rent.into(),
            payment: item.payment.into(),
        }
    }
}

impl From<event::ReceiptSent> for ReceiptSent {
    fn from(item: event::ReceiptSent) -> Self {
        Self {
            rent_id: item.rent_id,
        }
    }
}

impl From<event::StepCompleted> for StepCompleted {
    fn from(item: event::StepCompleted) -> Self {
        Self {
            step_id: item.step_id,
            requirements: item
                .requirements
                .map(|requirements| requirements.into_iter().map(Into::into).collect()),
        }
    }
}

impl From<event::StepCreated> for StepCreated {
    fn from(item: event::StepCreated) -> Self {
        Self {
            step: item.step.into(),
        }
    }
}

impl From<event::SubscriptionRequested> for SubscriptionRequested {
    fn from(item: event::SubscriptionRequested) -> Self {
        Self {
            account_id: item.account_id,
            email: item.email,
        }
    }
}

impl From<event::TenantCreated> for TenantCreated {
    fn from(item: event::TenantCreated) -> Self {
        Self {
            tenant: item.tenant.into(),
            identity: item.identity.map(Into::into),
            warrants: item
                .warrants
                .map(|warrants| warrants.into_iter().map(Into::into).collect()),
            discussion: item.discussion.map(Into::into),
        }
    }
}

impl From<event::TenantDeleted> for TenantDeleted {
    fn from(item: event::TenantDeleted) -> Self {
        Self {
            tenant_id: item.tenant_id,
        }
    }
}

impl From<event::TenantUpdated> for TenantUpdated {
    fn from(item: event::TenantUpdated) -> Self {
        Self {
            tenant: item.tenant.into(),
        }
    }
}

impl From<event::WarrantCreated> for WarrantCreated {
    fn from(item: event::WarrantCreated) -> Self {
        Self {
            warrant: item.warrant.into(),
        }
    }
}

impl From<event::WorkflowCreated> for WorkflowCreated {
    fn from(item: event::WorkflowCreated) -> Self {
        Self {
            workflow: item.workflow.into(),
            workflowable: item.workflowable.into(),
        }
    }
}
