use async_graphql::SimpleObject;
use async_graphql::Union;
use serde::Deserialize;
use serde::Serialize;
use std::fmt;
use trankeel_data::Account;
use trankeel_data::AccountId;
use trankeel_data::Advertisement;
use trankeel_data::AuthId;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyId;
use trankeel_data::Company;
use trankeel_data::Discussion;
use trankeel_data::DiscussionId;
use trankeel_data::Document;
use trankeel_data::Email;
use trankeel_data::EventType;
use trankeel_data::Invite;
use trankeel_data::InviteId;
use trankeel_data::Lease;
use trankeel_data::LeaseId;
use trankeel_data::LegalIdentity;
use trankeel_data::Lender;
use trankeel_data::Message;
use trankeel_data::Notice;
use trankeel_data::Payment;
use trankeel_data::Person;
use trankeel_data::Property;
use trankeel_data::PropertyId;
use trankeel_data::Receipt;
use trankeel_data::Rent;
use trankeel_data::RentId;
use trankeel_data::Step;
use trankeel_data::StepId;
use trankeel_data::Tenant;
use trankeel_data::TenantId;
use trankeel_data::WarrantWithIdentity;
use trankeel_data::Workflow;
use trankeel_data::Workflowable;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Serialize, Deserialize, Union)]
#[serde(tag = "event_type")]
#[serde(rename_all = "snake_case")]
pub enum Event {
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

impl Event {
    pub fn event_type(&self) -> EventType {
        match self {
            Event::AccountCreated(_) => EventType::AccountCreated,
            Event::AdvertisementCreated(_) => EventType::AdvertisementCreated,
            Event::AdvertisementUpdated(_) => EventType::AdvertisementUpdated,
            Event::CandidacyAccepted(_) => EventType::CandidacyAccepted,
            Event::CandidacyCreated(_) => EventType::CandidacyCreated,
            Event::CandidacyRejected(_) => EventType::CandidacyRejected,
            Event::CompanyCreated(_) => EventType::CompanyCreated,
            Event::DiscussionCreated(_) => EventType::DiscussionCreated,
            Event::DiscussionDeleted(_) => EventType::DiscussionDeleted,
            Event::DocumentGenerated(_) => EventType::DocumentGenerated,
            Event::InviteAccepted(_) => EventType::InviteAccepted,
            Event::InviteCreated(_) => EventType::InviteCreated,
            Event::LeaseAffected(_) => EventType::LeaseAffected,
            Event::LeaseCreated(_) => EventType::LeaseCreated,
            Event::LeaseDeleted(_) => EventType::LeaseDeleted,
            Event::LeaseFileRequested(_) => EventType::LeaseFileRequested,
            Event::LeaseUpdated(_) => EventType::LeaseUpdated,
            Event::LenderCreated(_) => EventType::LenderCreated,
            Event::LenderUpdated(_) => EventType::LenderUpdated,
            Event::MessagePushed(_) => EventType::MessagePushed,
            Event::NoticeCreated(_) => EventType::NoticeCreated,
            Event::PaymentCreated(_) => EventType::PaymentCreated,
            Event::PersonCreated(_) => EventType::PersonCreated,
            Event::PropertyCreated(_) => EventType::PropertyCreated,
            Event::PropertyDeleted(_) => EventType::PropertyDeleted,
            Event::PropertyUpdated(_) => EventType::PropertyUpdated,
            Event::ReceiptCreated(_) => EventType::ReceiptCreated,
            Event::ReceiptSent(_) => EventType::ReceiptSent,
            Event::StepCompleted(_) => EventType::StepCompleted,
            Event::StepCreated(_) => EventType::StepCreated,
            Event::SubscriptionRequested(_) => EventType::SubscriptionRequested,
            Event::TenantCreated(_) => EventType::TenantCreated,
            Event::TenantUpdated(_) => EventType::TenantUpdated,
            Event::TenantDeleted(_) => EventType::TenantDeleted,
            Event::WarrantCreated(_) => EventType::WarrantCreated,
            Event::WorkflowCreated(_) => EventType::WorkflowCreated,
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.event_type())
    }
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct AccountCreated {
    pub account: Account,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct AdvertisementCreated {
    pub advertisement: Advertisement,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct AdvertisementUpdated {
    pub advertisement: Advertisement,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct CandidacyAccepted {
    pub candidacy_id: CandidacyId,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct CandidacyCreated {
    pub candidacy: Candidacy,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct CandidacyRejected {
    pub candidacy_id: CandidacyId,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct CompanyCreated {
    pub company: Company,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct DiscussionCreated {
    pub discussion: Discussion,
    pub message: Option<Message>,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct DiscussionDeleted {
    pub discussion_id: DiscussionId,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct DocumentGenerated {
    pub document: Document,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct InviteAccepted {
    pub invite_id: InviteId,
    pub auth_id: AuthId,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct InviteCreated {
    pub invite: Invite,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct LeaseAffected {
    pub lease_id: LeaseId,
    pub tenant_id: TenantId,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct LeaseCreated {
    pub lease: Lease,
    pub rents: Vec<Rent>,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct LeaseDeleted {
    pub lease_id: LeaseId,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct LeaseFileRequested {
    pub lease_id: LeaseId,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct LeaseUpdated {
    pub lease: Lease,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct LenderCreated {
    pub lender: Lender,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct LenderUpdated {
    pub lender: Lender,
    pub identity: LegalIdentity,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct MessagePushed {
    pub message: Message,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct NoticeCreated {
    pub notice: Notice,
    pub rent: Rent,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct PaymentCreated {
    pub payment: Payment,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct PersonCreated {
    pub person: Person,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct PropertyCreated {
    pub property: Property,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct PropertyDeleted {
    pub property_id: PropertyId,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct PropertyUpdated {
    pub property: Property,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct ReceiptCreated {
    pub receipt: Receipt,
    pub rent: Rent,
    pub payment: Payment,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct ReceiptSent {
    pub rent_id: RentId,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct StepCompletedRequirement {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct StepCompleted {
    pub step_id: StepId,
    pub requirements: Option<Vec<StepCompletedRequirement>>,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct StepCreated {
    pub step: Step,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct SubscriptionRequested {
    pub account_id: AccountId,
    pub email: Email,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct TenantCreated {
    pub tenant: Tenant,
    pub identity: Option<Person>,
    pub warrants: Option<Vec<WarrantWithIdentity>>,
    pub discussion: Option<Discussion>,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct TenantDeleted {
    pub tenant_id: TenantId,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct TenantUpdated {
    pub tenant: Tenant,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct WarrantCreated {
    pub warrant: WarrantWithIdentity,
}

#[derive(Clone, Serialize, Deserialize, SimpleObject)]
pub struct WorkflowCreated {
    pub workflow: Workflow,
    pub workflowable: Workflowable,
}
