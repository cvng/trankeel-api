use crate::DomainEvent;
use std::fmt;
use trankeel_data::Account;
use trankeel_data::AccountId;
use trankeel_data::Advertisement;
use trankeel_data::AuthId;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyId;
use trankeel_data::Discussion;
use trankeel_data::DiscussionId;
use trankeel_data::Document;
use trankeel_data::Email;
use trankeel_data::Invite;
use trankeel_data::InviteId;
use trankeel_data::Lease;
use trankeel_data::LeaseFile;
use trankeel_data::Lender;
use trankeel_data::Message;
use trankeel_data::Notice;
use trankeel_data::Payment;
use trankeel_data::Person;
use trankeel_data::Property;
use trankeel_data::Receipt;
use trankeel_data::Rent;
use trankeel_data::RentId;
use trankeel_data::StepId;
use trankeel_data::Tenant;
use trankeel_data::WarrantWithIdentity;
use trankeel_data::Workflow;
use trankeel_data::Workflowable;

#[allow(clippy::large_enum_variant)]
#[derive(Clone)]
pub enum Event {
    AccountCreated(AccountCreated),
    AdvertisementCreated(AdvertisementCreated),
    AdvertisementUpdated(AdvertisementUpdated),
    CandidacyAccepted(CandidacyAccepted),
    CandidacyCreated(CandidacyCreated),
    CandidacyRejected(CandidacyRejected),
    DiscussionCreated(DiscussionCreated),
    DiscussionDeleted(DiscussionDeleted),
    DocumentGenerated(DocumentGenerated),
    InviteAccepted(InviteAccepted),
    LeaseAffected(LeaseAffected),
    LeaseCreated(LeaseCreated),
    LenderCreated(LenderCreated),
    MessagePushed(MessagePushed),
    NoticeCreated(NoticeCreated),
    PaymentCreated(PaymentCreated),
    PersonCreated(PersonCreated),
    PropertyCreated(PropertyCreated),
    PropertyUpdated(PropertyUpdated),
    ReceiptCreated(ReceiptCreated),
    ReceiptSent(ReceiptSent),
    StepCompleted(StepCompleted),
    SubscriptionRequested(SubscriptionRequested),
    TenantCreated(TenantCreated),
    TenantUpdated(TenantUpdated),
    WarrantCreated(WarrantCreated),
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Event::AccountCreated(_) => "account_created",
                Event::AdvertisementCreated(_) => "advertisement_created",
                Event::AdvertisementUpdated(_) => "advertisement_updated",
                Event::CandidacyAccepted(_) => "candidacy_accepted",
                Event::CandidacyCreated(_) => "candidacy_created",
                Event::CandidacyRejected(_) => "candidacy_rejected",
                Event::DiscussionCreated(_) => "discussion_created",
                Event::DiscussionDeleted(_) => "discussion_deleted",
                Event::DocumentGenerated(_) => "document_generated",
                Event::InviteAccepted(_) => "invite_accepted",
                Event::LeaseAffected(_) => "lease_affected",
                Event::LeaseCreated(_) => "lease_created",
                Event::LenderCreated(_) => "lender_created",
                Event::MessagePushed(_) => "message_pushed",
                Event::NoticeCreated(_) => "notice_created",
                Event::PaymentCreated(_) => "payment_created",
                Event::PersonCreated(_) => "person_created",
                Event::PropertyCreated(_) => "property_created",
                Event::PropertyUpdated(_) => "property_updated",
                Event::ReceiptCreated(_) => "receipt_created",
                Event::ReceiptSent(_) => "receipt_sent",
                Event::StepCompleted(_) => "step_completed",
                Event::SubscriptionRequested(_) => "subscription_requested",
                Event::TenantCreated(_) => "tenant_created",
                Event::TenantUpdated(_) => "tenant_updated",
                Event::WarrantCreated(_) => "warrant_created",
            }
        )
    }
}

impl DomainEvent for AccountCreated {}

impl DomainEvent for AdvertisementCreated {}

impl DomainEvent for AdvertisementUpdated {}

impl DomainEvent for CandidacyAccepted {}

impl DomainEvent for CandidacyCreated {}

impl DomainEvent for CandidacyRejected {}

impl DomainEvent for DiscussionCreated {}

impl DomainEvent for DiscussionDeleted {}

impl DomainEvent for DocumentGenerated {}

impl DomainEvent for InviteAccepted {}

impl DomainEvent for LeaseAffected {}

impl DomainEvent for LeaseCreated {}

impl DomainEvent for LenderCreated {}

impl DomainEvent for MessagePushed {}

impl DomainEvent for NoticeCreated {}

impl DomainEvent for PaymentCreated {}

impl DomainEvent for PersonCreated {}

impl DomainEvent for PropertyCreated {}

impl DomainEvent for PropertyUpdated {}

impl DomainEvent for ReceiptCreated {}

impl DomainEvent for ReceiptSent {}

impl DomainEvent for StepCompleted {}

impl DomainEvent for SubscriptionRequested {}

impl DomainEvent for TenantCreated {}

impl DomainEvent for TenantUpdated {}

impl DomainEvent for WarrantCreated {}

#[derive(Clone)]
pub struct AccountCreated {
    pub account: Account,
}

impl From<AccountCreated> for Event {
    fn from(item: AccountCreated) -> Self {
        Self::AccountCreated(item)
    }
}

#[derive(Clone)]
pub struct AdvertisementCreated {
    pub advertisement: Advertisement,
}

impl From<AdvertisementCreated> for Event {
    fn from(item: AdvertisementCreated) -> Self {
        Self::AdvertisementCreated(item)
    }
}

#[derive(Clone)]
pub struct AdvertisementUpdated {
    pub advertisement: Advertisement,
}

impl From<AdvertisementUpdated> for Event {
    fn from(item: AdvertisementUpdated) -> Self {
        Self::AdvertisementUpdated(item)
    }
}

#[derive(Clone)]
pub struct CandidacyAccepted {
    pub candidacy: Candidacy,
    pub rejected_candidacies: Vec<Candidacy>,
    pub tenant: Tenant,
    pub identity: Person,
    pub warrants: Option<Vec<WarrantWithIdentity>>,
    pub discussion: Discussion,
    pub lease: Lease,
    pub rents: Vec<Rent>,
    pub lease_file: LeaseFile,
    pub workflow: Workflow,
    pub workflowable: Workflowable,
    pub invite: Invite,
}

impl From<CandidacyAccepted> for Event {
    fn from(item: CandidacyAccepted) -> Self {
        Self::CandidacyAccepted(item)
    }
}

#[derive(Clone)]
pub struct CandidacyCreated {
    pub candidacy: Candidacy,
}

impl From<CandidacyCreated> for Event {
    fn from(item: CandidacyCreated) -> Self {
        Self::CandidacyCreated(item)
    }
}

#[derive(Clone)]
pub struct CandidacyRejected {
    pub candidacy_id: CandidacyId,
}

impl From<CandidacyRejected> for Event {
    fn from(item: CandidacyRejected) -> Self {
        Self::CandidacyRejected(item)
    }
}

#[derive(Clone)]
pub struct DiscussionCreated {
    pub discussion: Discussion,
}

impl From<DiscussionCreated> for Event {
    fn from(item: DiscussionCreated) -> Self {
        Self::DiscussionCreated(item)
    }
}

#[derive(Clone)]
pub struct DiscussionDeleted {
    pub discussion_id: DiscussionId,
}

impl From<DiscussionDeleted> for Event {
    fn from(item: DiscussionDeleted) -> Self {
        Self::DiscussionDeleted(item)
    }
}

#[derive(Clone, Debug)]
pub struct DocumentGenerated {
    pub document: Document,
}

impl From<DocumentGenerated> for Event {
    fn from(item: DocumentGenerated) -> Self {
        Self::DocumentGenerated(item)
    }
}

#[derive(Clone, Debug)]
pub struct InviteAccepted {
    pub invite_id: InviteId,
    pub auth_id: AuthId,
}

impl From<InviteAccepted> for Event {
    fn from(item: InviteAccepted) -> Self {
        Self::InviteAccepted(item)
    }
}

#[derive(Clone)]
pub struct LeaseAffected {
    pub tenant: Tenant,
}

impl From<LeaseAffected> for Event {
    fn from(item: LeaseAffected) -> Self {
        Self::LeaseAffected(item)
    }
}

#[derive(Clone)]
pub struct LeaseCreated {
    pub lease: Lease,
    pub rents: Vec<Rent>,
}

impl From<LeaseCreated> for Event {
    fn from(item: LeaseCreated) -> Self {
        Self::LeaseCreated(item)
    }
}

#[derive(Clone)]
pub struct LenderCreated {
    pub lender: Lender,
}

impl From<LenderCreated> for Event {
    fn from(item: LenderCreated) -> Self {
        Self::LenderCreated(item)
    }
}

#[derive(Clone)]
pub struct MessagePushed {
    pub message: Message,
}

impl From<MessagePushed> for Event {
    fn from(item: MessagePushed) -> Self {
        Self::MessagePushed(item)
    }
}

#[derive(Clone)]
pub struct NoticeCreated {
    pub notice: Notice,
    pub rent: Rent,
}

impl From<NoticeCreated> for Event {
    fn from(item: NoticeCreated) -> Self {
        Self::NoticeCreated(item)
    }
}

#[derive(Clone)]
pub struct PaymentCreated {
    pub payment: Payment,
}

impl From<PaymentCreated> for Event {
    fn from(item: PaymentCreated) -> Self {
        Self::PaymentCreated(item)
    }
}

#[derive(Clone)]
pub struct PersonCreated {
    pub person: Person,
}

impl From<PersonCreated> for Event {
    fn from(item: PersonCreated) -> Self {
        Self::PersonCreated(item)
    }
}

#[derive(Clone)]
pub struct PropertyCreated {
    pub property: Property,
}

impl From<PropertyCreated> for Event {
    fn from(item: PropertyCreated) -> Self {
        Self::PropertyCreated(item)
    }
}

#[derive(Clone)]
pub struct PropertyUpdated {
    pub property: Property,
}

impl From<PropertyUpdated> for Event {
    fn from(item: PropertyUpdated) -> Self {
        Self::PropertyUpdated(item)
    }
}

#[derive(Clone)]
pub struct ReceiptCreated {
    pub receipt: Receipt,
    pub rent: Rent,
    pub payment: Payment,
}

impl From<ReceiptCreated> for Event {
    fn from(item: ReceiptCreated) -> Self {
        Self::ReceiptCreated(item)
    }
}

#[derive(Clone)]
pub struct ReceiptSent {
    pub rent_id: RentId,
}

impl From<ReceiptSent> for Event {
    fn from(item: ReceiptSent) -> Self {
        Self::ReceiptSent(item)
    }
}

#[derive(Clone)]
pub struct StepCompletedRequirement {
    pub name: String,
    pub value: String,
}

#[derive(Clone)]
pub struct StepCompleted {
    pub step_id: StepId,
    pub requirements: Option<Vec<StepCompletedRequirement>>,
}

impl From<StepCompleted> for Event {
    fn from(item: StepCompleted) -> Self {
        Self::StepCompleted(item)
    }
}

#[derive(Clone)]
pub struct SubscriptionRequested {
    pub account_id: AccountId,
    pub email: Email,
}

impl From<SubscriptionRequested> for Event {
    fn from(item: SubscriptionRequested) -> Self {
        Self::SubscriptionRequested(item)
    }
}

#[derive(Clone)]
pub struct TenantCreated {
    pub tenant: Tenant,
    pub identity: Person,
    pub discussion: Option<Discussion>,
    pub warrants: Option<Vec<WarrantWithIdentity>>,
}

impl From<TenantCreated> for Event {
    fn from(item: TenantCreated) -> Self {
        Self::TenantCreated(item)
    }
}

#[derive(Clone)]
pub struct TenantUpdated {
    pub tenant: Tenant,
}

impl From<TenantUpdated> for Event {
    fn from(item: TenantUpdated) -> Self {
        Self::TenantUpdated(item)
    }
}

#[derive(Clone)]
pub struct WarrantCreated {
    pub warrant: WarrantWithIdentity,
}

impl From<WarrantCreated> for Event {
    fn from(item: WarrantCreated) -> Self {
        Self::WarrantCreated(item)
    }
}
