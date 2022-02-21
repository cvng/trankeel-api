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
#[derive(Clone, Serialize)]
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
    pub fn event_type(&self) -> String {
        match self {
            Event::AccountCreated(_) => "account_created".into(),
            Event::AdvertisementCreated(_) => "advertisement_created".into(),
            Event::AdvertisementUpdated(_) => "advertisement_updated".into(),
            Event::CandidacyAccepted(_) => "candidacy_accepted".into(),
            Event::CandidacyCreated(_) => "candidacy_created".into(),
            Event::CandidacyRejected(_) => "candidacy_rejected".into(),
            Event::CompanyCreated(_) => "company_created".into(),
            Event::DiscussionCreated(_) => "discussion_created".into(),
            Event::DiscussionDeleted(_) => "discussion_deleted".into(),
            Event::DocumentGenerated(_) => "document_generated".into(),
            Event::InviteAccepted(_) => "invite_accepted".into(),
            Event::InviteCreated(_) => "invite_created".into(),
            Event::LeaseAffected(_) => "lease_affected".into(),
            Event::LeaseCreated(_) => "lease_created".into(),
            Event::LeaseDeleted(_) => "lease_deleted".into(),
            Event::LeaseFileRequested(_) => "lease_file_requested".into(),
            Event::LeaseUpdated(_) => "lease_updated".into(),
            Event::LenderCreated(_) => "lender_created".into(),
            Event::LenderUpdated(_) => "lender_updated".into(),
            Event::MessagePushed(_) => "message_pushed".into(),
            Event::NoticeCreated(_) => "notice_created".into(),
            Event::PaymentCreated(_) => "payment_created".into(),
            Event::PersonCreated(_) => "person_created".into(),
            Event::PropertyCreated(_) => "property_created".into(),
            Event::PropertyDeleted(_) => "property_deleted".into(),
            Event::PropertyUpdated(_) => "property_updated".into(),
            Event::ReceiptCreated(_) => "receipt_created".into(),
            Event::ReceiptSent(_) => "receipt_sent".into(),
            Event::StepCompleted(_) => "step_completed".into(),
            Event::StepCreated(_) => "step_created".into(),
            Event::SubscriptionRequested(_) => "subscription_requested".into(),
            Event::TenantCreated(_) => "tenant_created".into(),
            Event::TenantUpdated(_) => "tenant_updated".into(),
            Event::TenantDeleted(_) => "tenant_deleted".into(),
            Event::WarrantCreated(_) => "warrant_created".into(),
            Event::WorkflowCreated(_) => "workflow_created".into(),
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.event_type())
    }
}

#[derive(Clone, Serialize)]
pub struct AccountCreated {
    pub account: Account,
}

impl From<AccountCreated> for Event {
    fn from(item: AccountCreated) -> Self {
        Self::AccountCreated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct AdvertisementCreated {
    pub advertisement: Advertisement,
}

impl From<AdvertisementCreated> for Event {
    fn from(item: AdvertisementCreated) -> Self {
        Self::AdvertisementCreated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct AdvertisementUpdated {
    pub advertisement: Advertisement,
}

impl From<AdvertisementUpdated> for Event {
    fn from(item: AdvertisementUpdated) -> Self {
        Self::AdvertisementUpdated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct CandidacyAccepted {
    pub candidacy_id: CandidacyId,
}

impl From<CandidacyAccepted> for Event {
    fn from(item: CandidacyAccepted) -> Self {
        Self::CandidacyAccepted(item)
    }
}

#[derive(Clone, Serialize)]
pub struct CandidacyCreated {
    pub candidacy: Candidacy,
}

impl From<CandidacyCreated> for Event {
    fn from(item: CandidacyCreated) -> Self {
        Self::CandidacyCreated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct CandidacyRejected {
    pub candidacy_id: CandidacyId,
}

impl From<CandidacyRejected> for Event {
    fn from(item: CandidacyRejected) -> Self {
        Self::CandidacyRejected(item)
    }
}

#[derive(Clone, Serialize)]
pub struct CompanyCreated {
    pub company: Company,
}

impl From<CompanyCreated> for Event {
    fn from(item: CompanyCreated) -> Self {
        Self::CompanyCreated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct DiscussionCreated {
    pub discussion: Discussion,
    pub message: Option<Message>,
}

impl From<DiscussionCreated> for Event {
    fn from(item: DiscussionCreated) -> Self {
        Self::DiscussionCreated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct DiscussionDeleted {
    pub discussion_id: DiscussionId,
}

impl From<DiscussionDeleted> for Event {
    fn from(item: DiscussionDeleted) -> Self {
        Self::DiscussionDeleted(item)
    }
}

#[derive(Clone, Serialize)]
pub struct DocumentGenerated {
    pub document: Document,
}

impl From<DocumentGenerated> for Event {
    fn from(item: DocumentGenerated) -> Self {
        Self::DocumentGenerated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct InviteAccepted {
    pub invite_id: InviteId,
    pub auth_id: AuthId,
}

impl From<InviteAccepted> for Event {
    fn from(item: InviteAccepted) -> Self {
        Self::InviteAccepted(item)
    }
}

#[derive(Clone, Serialize)]
pub struct InviteCreated {
    pub invite: Invite,
}

impl From<InviteCreated> for Event {
    fn from(item: InviteCreated) -> Self {
        Self::InviteCreated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct LeaseAffected {
    pub lease_id: LeaseId,
    pub tenant_id: TenantId,
}

impl From<LeaseAffected> for Event {
    fn from(item: LeaseAffected) -> Self {
        Self::LeaseAffected(item)
    }
}

#[derive(Clone, Serialize)]
pub struct LeaseCreated {
    pub lease: Lease,
    pub rents: Vec<Rent>,
}

impl From<LeaseCreated> for Event {
    fn from(item: LeaseCreated) -> Self {
        Self::LeaseCreated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct LeaseDeleted {
    pub lease_id: LeaseId,
}

impl From<LeaseDeleted> for Event {
    fn from(item: LeaseDeleted) -> Self {
        Self::LeaseDeleted(item)
    }
}

#[derive(Clone, Serialize)]
pub struct LeaseFileRequested {
    pub lease_id: LeaseId,
}

impl From<LeaseFileRequested> for Event {
    fn from(item: LeaseFileRequested) -> Self {
        Self::LeaseFileRequested(item)
    }
}

#[derive(Clone, Serialize)]
pub struct LeaseUpdated {
    pub lease: Lease,
}

impl From<LeaseUpdated> for Event {
    fn from(item: LeaseUpdated) -> Self {
        Self::LeaseUpdated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct LenderCreated {
    pub lender: Lender,
}

impl From<LenderCreated> for Event {
    fn from(item: LenderCreated) -> Self {
        Self::LenderCreated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct LenderUpdated {
    pub lender: Lender,
    pub identity: LegalIdentity,
}

impl From<LenderUpdated> for Event {
    fn from(item: LenderUpdated) -> Self {
        Self::LenderUpdated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct MessagePushed {
    pub message: Message,
}

impl From<MessagePushed> for Event {
    fn from(item: MessagePushed) -> Self {
        Self::MessagePushed(item)
    }
}

#[derive(Clone, Serialize)]
pub struct NoticeCreated {
    pub notice: Notice,
    pub rent: Rent,
}

impl From<NoticeCreated> for Event {
    fn from(item: NoticeCreated) -> Self {
        Self::NoticeCreated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct PaymentCreated {
    pub payment: Payment,
}

impl From<PaymentCreated> for Event {
    fn from(item: PaymentCreated) -> Self {
        Self::PaymentCreated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct PersonCreated {
    pub person: Person,
}

impl From<PersonCreated> for Event {
    fn from(item: PersonCreated) -> Self {
        Self::PersonCreated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct PropertyCreated {
    pub property: Property,
}

impl From<PropertyCreated> for Event {
    fn from(item: PropertyCreated) -> Self {
        Self::PropertyCreated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct PropertyDeleted {
    pub property_id: PropertyId,
}

impl From<PropertyDeleted> for Event {
    fn from(item: PropertyDeleted) -> Self {
        Self::PropertyDeleted(item)
    }
}

#[derive(Clone, Serialize)]
pub struct PropertyUpdated {
    pub property: Property,
}

impl From<PropertyUpdated> for Event {
    fn from(item: PropertyUpdated) -> Self {
        Self::PropertyUpdated(item)
    }
}

#[derive(Clone, Serialize)]
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

#[derive(Clone, Serialize)]
pub struct ReceiptSent {
    pub rent_id: RentId,
}

impl From<ReceiptSent> for Event {
    fn from(item: ReceiptSent) -> Self {
        Self::ReceiptSent(item)
    }
}

#[derive(Clone, Serialize)]
pub struct StepCompletedRequirement {
    pub name: String,
    pub value: String,
}

#[derive(Clone, Serialize)]
pub struct StepCompleted {
    pub step_id: StepId,
    pub requirements: Option<Vec<StepCompletedRequirement>>,
}

impl From<StepCompleted> for Event {
    fn from(item: StepCompleted) -> Self {
        Self::StepCompleted(item)
    }
}

#[derive(Clone, Serialize)]
pub struct StepCreated {
    pub step: Step,
}

impl From<StepCreated> for Event {
    fn from(item: StepCreated) -> Self {
        Self::StepCreated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct SubscriptionRequested {
    pub account_id: AccountId,
    pub email: Email,
}

impl From<SubscriptionRequested> for Event {
    fn from(item: SubscriptionRequested) -> Self {
        Self::SubscriptionRequested(item)
    }
}

#[derive(Clone, Serialize)]
pub struct TenantCreated {
    pub tenant: Tenant,
    pub identity: Option<Person>,
    pub warrants: Option<Vec<WarrantWithIdentity>>,
    pub discussion: Option<Discussion>,
}

impl From<TenantCreated> for Event {
    fn from(item: TenantCreated) -> Self {
        Self::TenantCreated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct TenantDeleted {
    pub tenant_id: TenantId,
}

impl From<TenantDeleted> for Event {
    fn from(item: TenantDeleted) -> Self {
        Self::TenantDeleted(item)
    }
}

#[derive(Clone, Serialize)]
pub struct TenantUpdated {
    pub tenant: Tenant,
}

impl From<TenantUpdated> for Event {
    fn from(item: TenantUpdated) -> Self {
        Self::TenantUpdated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct WarrantCreated {
    pub warrant: WarrantWithIdentity,
}

impl From<WarrantCreated> for Event {
    fn from(item: WarrantCreated) -> Self {
        Self::WarrantCreated(item)
    }
}

#[derive(Clone, Serialize)]
pub struct WorkflowCreated {
    pub workflow: Workflow,
    pub workflowable: Workflowable,
}

impl From<WorkflowCreated> for Event {
    fn from(item: WorkflowCreated) -> Self {
        Self::WorkflowCreated(item)
    }
}
