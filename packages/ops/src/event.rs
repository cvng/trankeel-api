use crate::DomainEvent;
use trankeel_data::Advertisement;
use trankeel_data::Candidacy;
use trankeel_data::Discussion;
use trankeel_data::DiscussionId;
use trankeel_data::Document;
use trankeel_data::Invite;
use trankeel_data::Lease;
use trankeel_data::LeaseFile;
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
    AdvertisementCreated(AdvertisementCreated),
    AdvertisementUpdated(AdvertisementUpdated),
    CandidacyAccepted(CandidacyAccepted),
    CandidacyCreated(CandidacyCreated),
    CandidacyRejected(CandidacyRejected),
    DiscussionDeleted(DiscussionDeleted),
    DocumentGenerated(DocumentGenerated),
    LeaseAffected(LeaseAffected),
    LeaseCreated(LeaseCreated),
    MessagePushed(MessagePushed),
    NoticeCreated(NoticeCreated),
    PaymentCreated(PaymentCreated),
    PersonCreated(PersonCreated),
    PropertyCreated(PropertyCreated),
    PropertyUpdated(PropertyUpdated),
    ReceiptCreated(ReceiptCreated),
    ReceiptSent(ReceiptSent),
    StepCompleted(StepCompleted),
    TenantCreated(TenantCreated),
    TenantUpdated(TenantUpdated),
}

impl DomainEvent for AdvertisementCreated {}

impl DomainEvent for AdvertisementUpdated {}

impl DomainEvent for CandidacyAccepted {}

impl DomainEvent for CandidacyCreated {}

impl DomainEvent for CandidacyRejected {}

impl DomainEvent for DiscussionDeleted {}

impl DomainEvent for DocumentGenerated {}

impl DomainEvent for LeaseAffected {}

impl DomainEvent for LeaseCreated {}

impl DomainEvent for MessagePushed {}

impl DomainEvent for NoticeCreated {}

impl DomainEvent for PaymentCreated {}

impl DomainEvent for PersonCreated {}

impl DomainEvent for PropertyCreated {}

impl DomainEvent for PropertyUpdated {}

impl DomainEvent for ReceiptCreated {}

impl DomainEvent for ReceiptSent {}

impl DomainEvent for StepCompleted {}

impl DomainEvent for TenantCreated {}

impl DomainEvent for TenantUpdated {}

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
    pub rejected_candidacies: Vec<(Candidacy, (Discussion, Message))>,
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
    pub candidacy: Candidacy,
    pub discussion: Discussion,
    pub message: Message,
}

impl From<CandidacyRejected> for Event {
    fn from(item: CandidacyRejected) -> Self {
        Self::CandidacyRejected(item)
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
