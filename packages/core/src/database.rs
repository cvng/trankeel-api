use crate::error::Error;
use trankeel_data::Account;
use trankeel_data::AccountId;
use trankeel_data::Advertisement;
use trankeel_data::AdvertisementId;
use trankeel_data::AuthId;
use trankeel_data::Balance;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyId;
use trankeel_data::Company;
use trankeel_data::CompanyId;
use trankeel_data::Discussion;
use trankeel_data::DiscussionId;
use trankeel_data::DiscussionItem;
use trankeel_data::Event;
use trankeel_data::EventId;
use trankeel_data::EventWithEventable;
use trankeel_data::Eventable;
use trankeel_data::ExternalId;
use trankeel_data::File;
use trankeel_data::FileId;
use trankeel_data::Invite;
use trankeel_data::InviteToken;
use trankeel_data::Lease;
use trankeel_data::LeaseFileId;
use trankeel_data::LeaseId;
use trankeel_data::Lender;
use trankeel_data::LenderId;
use trankeel_data::LenderWithIdentity;
use trankeel_data::Message;
use trankeel_data::NoticeId;
use trankeel_data::Payment;
use trankeel_data::PaymentId;
use trankeel_data::Person;
use trankeel_data::PersonId;
use trankeel_data::Plan;
use trankeel_data::PlanId;
use trankeel_data::Property;
use trankeel_data::PropertyId;
use trankeel_data::ReceiptId;
use trankeel_data::Rent;
use trankeel_data::RentId;
use trankeel_data::Step;
use trankeel_data::StepId;
use trankeel_data::Summary;
use trankeel_data::Tenant;
use trankeel_data::TenantId;
use trankeel_data::TenantWithBalance;
use trankeel_data::Warrant;
use trankeel_data::WarrantId;
use trankeel_data::WarrantWithIdentity;
use trankeel_data::Workflow;
use trankeel_data::WorkflowId;
use trankeel_data::WorkflowWithSteps;
use trankeel_data::Workflowable;
use trankeel_data::WorkflowableId;

pub type Result<T> = std::result::Result<T, Error>;

pub type Executed = usize;

pub trait Db {
    fn accounts(&self) -> Box<dyn AccountStore + '_>;
    fn balances(&self) -> Box<dyn BalanceStore + '_>;
    fn persons(&self) -> Box<dyn PersonStore + '_>;
    fn companies(&self) -> Box<dyn CompanyStore + '_>;
    fn lenders(&self) -> Box<dyn LenderStore + '_>;
    fn tenants(&self) -> Box<dyn TenantStore + '_>;
    fn warrants(&self) -> Box<dyn WarrantStore + '_>;
    fn advertisements(&self) -> Box<dyn AdvertisementStore + '_>;
    fn candidacies(&self) -> Box<dyn CandidacyStore + '_>;
    fn properties(&self) -> Box<dyn PropertyStore + '_>;
    fn leases(&self) -> Box<dyn LeaseStore + '_>;
    fn rents(&self) -> Box<dyn RentStore + '_>;
    fn files(&self) -> Box<dyn FileStore + '_>;
    fn payments(&self) -> Box<dyn PaymentStore + '_>;
    fn plans(&self) -> Box<dyn PlanStore + '_>;
    fn events(&self) -> Box<dyn EventStore + '_>;
    fn eventables(&self) -> Box<dyn EventableStore + '_>;
    fn reports(&self) -> Box<dyn ReportStore + '_>;
    fn discussions(&self) -> Box<dyn DiscussionStore + '_>;
    fn messages(&self) -> Box<dyn MessageStore + '_>;
    fn invites(&self) -> Box<dyn InviteStore + '_>;
    fn workflowables(&self) -> Box<dyn WorkflowableStore + '_>;
    fn workflows(&self) -> Box<dyn WorkflowStore + '_>;
    fn steps(&self) -> Box<dyn StepStore + '_>;
}

pub trait AccountStore {
    fn by_id(&mut self, id: &AccountId) -> Result<Account>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Account>;
    fn by_advertisement_id(&mut self, advertisement_id: &AdvertisementId) -> Result<Account>;
    fn by_candidacy_id(&mut self, candidacy_id: &CandidacyId) -> Result<Account>;
    fn by_notice_id(&mut self, notice_id: &NoticeId) -> Result<Account>;
    fn by_receipt_id(&mut self, receipt_id: &ReceiptId) -> Result<Account>;
    fn by_payment_id(&mut self, payment_id: &PaymentId) -> Result<Account>;
    fn by_person_id(&mut self, person_id: &PersonId) -> Result<Account>;
    fn by_lease_id(&mut self, lease_id: &LeaseId) -> Result<Account>;
    fn by_step_id(&mut self, step_id: &StepId) -> Result<Account>;
    fn create(&mut self, data: &Account) -> Result<Account>;
    fn update(&mut self, data: &Account) -> Result<Account>;
}

pub trait BalanceStore {
    fn by_tenant_id(&mut self, tenant_id: &TenantId) -> Result<Balance>;
}

pub trait PersonStore {
    fn by_id(&mut self, id: &PersonId) -> Result<Person>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Person>;
    fn by_account_id(&mut self, account_id: &AccountId) -> Result<Vec<Person>>;
    fn by_account_id_first(&mut self, account_id: &AccountId) -> Result<Person>;
    fn by_candidacy_id(&mut self, candidacy_id: &CandidacyId) -> Result<Person>;
    fn by_notice_id(&mut self, notice_id: &NoticeId) -> Result<Person>;
    fn by_receipt_id(&mut self, receipt_id: &ReceiptId) -> Result<Person>;
    fn by_payment_id(&mut self, payment_id: &PaymentId) -> Result<Person>;
    fn by_lease_id(&mut self, lease_id: &LeaseId) -> Result<Person>;
    fn by_step_id(&mut self, step_id: &StepId) -> Result<Person>;
    fn by_tenant_id(&mut self, tenant_id: &TenantId) -> Result<Person>;
    fn create(&mut self, data: &Person) -> Result<Person>;
    fn create_many(&mut self, data: &[Person]) -> Result<Vec<Person>>;
    fn update(&mut self, data: &Person) -> Result<Person>;
}

pub trait CompanyStore {
    fn by_id(&mut self, id: &CompanyId) -> Result<Company>;
}

pub trait LenderStore {
    fn by_id(&mut self, id: &LenderId) -> Result<LenderWithIdentity>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<LenderWithIdentity>>;
    fn by_account_id(&mut self, account_id: &AccountId) -> Result<Vec<LenderWithIdentity>>;
    fn by_account_id_first(&mut self, account_id: &AccountId) -> Result<LenderWithIdentity>;
    fn by_individual_id(&mut self, individual_id: &PersonId) -> Result<LenderWithIdentity>;
    fn create(&mut self, data: &Lender) -> Result<Lender>;
    fn update(&mut self, data: &Lender) -> Result<Lender>;
}

pub trait AdvertisementStore {
    fn by_id(&mut self, id: &AdvertisementId) -> Result<Advertisement>;
    fn by_id_published(&mut self, id: &AdvertisementId) -> Result<Advertisement>;
    fn by_candidacy_id(&mut self, candidacy_id: &CandidacyId) -> Result<Advertisement>;
    fn by_property_id(&mut self, property_id: &PropertyId) -> Result<Vec<Advertisement>>;
    fn create(&mut self, data: &Advertisement) -> Result<Advertisement>;
    fn update(&mut self, data: &Advertisement) -> Result<Advertisement>;
}

pub trait CandidacyStore {
    fn by_id(&mut self, id: &CandidacyId) -> Result<Candidacy>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Candidacy>>;
    fn by_advertisement_id(&mut self, advertisement_id: &AdvertisementId)
        -> Result<Vec<Candidacy>>;
    fn by_property_id(&mut self, property_id: &PropertyId) -> Result<Vec<Candidacy>>;
    fn by_person_id(&mut self, person_id: &PersonId) -> Result<Candidacy>;
    fn create(&mut self, data: &Candidacy) -> Result<Candidacy>;
    fn update(&mut self, data: &Candidacy) -> Result<Candidacy>;
}

pub trait PropertyStore {
    fn by_id(&mut self, id: &PropertyId) -> Result<Property>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Property>>;
    fn by_advertisement_id(&mut self, advertisement_id: &AdvertisementId) -> Result<Property>;
    fn create(&mut self, data: &Property) -> Result<Property>;
    fn delete(&mut self, data: &PropertyId) -> Result<Executed>;
    fn update(&mut self, data: &Property) -> Result<Property>;
}

pub trait TenantStore {
    fn by_id(&mut self, id: &TenantId) -> Result<Tenant>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Tenant>>;
    fn by_id_with_balance(&mut self, id: &TenantId) -> Result<TenantWithBalance>;
    fn by_auth_id_with_balances(&mut self, auth_id: &AuthId) -> Result<Vec<TenantWithBalance>>;
    fn by_lease_id(&mut self, lease_id: &LeaseId) -> Result<Vec<Tenant>>;
    fn by_person_id(&mut self, person_id: &PersonId) -> Result<Tenant>;
    fn create(&mut self, data: &Tenant) -> Result<Tenant>;
    fn create_many(&mut self, data: &[Tenant]) -> Result<Vec<Tenant>>;
    fn delete(&mut self, data: &TenantId) -> Result<Executed>;
    fn update(&mut self, data: &Tenant) -> Result<Tenant>;
}

pub trait WarrantStore {
    fn by_id(&mut self, id: &WarrantId) -> Result<WarrantWithIdentity>;
    fn by_candidacy_id(&mut self, candidacy_id: &CandidacyId) -> Result<Vec<WarrantWithIdentity>>;
    fn by_tenant_id(&mut self, tenant_id: &TenantId) -> Result<Vec<WarrantWithIdentity>>;
    fn create(&mut self, data: &WarrantWithIdentity) -> Result<WarrantWithIdentity>;
    fn create_many(&mut self, data: &[WarrantWithIdentity]) -> Result<Vec<WarrantWithIdentity>>;
    fn with_identity(&mut self, data: Warrant) -> Result<WarrantWithIdentity>;
}

pub trait LeaseStore {
    fn by_id(&mut self, id: &LeaseId) -> Result<Lease>;
    fn by_property_id(&mut self, property_id: &PropertyId) -> Result<Vec<Lease>>;
    fn by_lease_file_id(&mut self, lease_file: &LeaseFileId) -> Result<Lease>;
    fn by_receipt_id(&mut self, receipt_id: &ReceiptId) -> Result<Lease>;
    fn by_notice_id(&mut self, notice_id: &NoticeId) -> Result<Lease>;
    fn by_rent_id(&mut self, rent_id: &RentId) -> Result<Lease>;
    fn by_tenant_id(&mut self, tenant_id: &TenantId) -> Result<Lease>;
    fn by_person_id(&mut self, person_id: &PersonId) -> Result<Lease>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Lease>>;
    fn create(&mut self, data: &Lease) -> Result<Lease>;
    fn delete(&mut self, data: &LeaseId) -> Result<Executed>;
    fn update(&mut self, data: &Lease) -> Result<Lease>;
}

pub trait RentStore {
    fn by_id(&mut self, id: &RentId) -> Result<Rent>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Rent>>;
    fn by_receipt_id(&mut self, receipt_id: &ReceiptId) -> Result<Rent>;
    fn by_notice_id(&mut self, notice_id: &NoticeId) -> Result<Rent>;
    fn by_lease_id(&mut self, lease_id: &LeaseId) -> Result<Vec<Rent>>;
    fn create_many(&mut self, data: &[Rent]) -> Result<Vec<Rent>>;
    fn update(&mut self, data: &Rent) -> Result<Rent>;
}

pub trait FileStore {
    fn by_id(&mut self, id: &FileId) -> Result<File>;
    #[allow(clippy::ptr_arg)]
    fn by_external_id(&mut self, external_id: &ExternalId) -> Result<File>;
    fn create(&mut self, data: &File) -> Result<File>;
    fn update(&mut self, data: &File) -> Result<File>;
}

pub trait PaymentStore {
    fn create(&mut self, data: &Payment) -> Result<Payment>;
    fn by_rent_id(&mut self, rent_id: &RentId) -> Result<Vec<Payment>>;
}

pub trait PlanStore {
    fn by_id(&mut self, id: &PlanId) -> Result<Plan>;
}

pub trait EventStore {
    fn by_id(&mut self, id: &EventId) -> Result<EventWithEventable>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<EventWithEventable>>;
    fn create(&mut self, data: &Event) -> Result<Event>;
}

pub trait EventableStore {
    fn create(&mut self, data: &Eventable) -> Result<Eventable>;
}

pub trait ReportStore {
    fn by_account_id(&mut self, account_id: &AccountId) -> Result<Summary>;
}

pub trait DiscussionStore {
    fn by_id(&mut self, id: &DiscussionId) -> Result<Discussion>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Discussion>>;
    fn by_initiator_id(&mut self, person_id: &PersonId) -> Result<Discussion>;
    fn by_candidacy_id(&mut self, candidacy_id: &CandidacyId) -> Result<Discussion>;
    fn create(&mut self, data: &Discussion) -> Result<Discussion>;
    fn create_many(&mut self, data: &[Discussion]) -> Result<Vec<Discussion>>;
    fn update(&mut self, data: &Discussion) -> Result<Discussion>;
    fn delete(&mut self, data: &DiscussionId) -> Result<Executed>;
    fn related_items(&mut self, id: &DiscussionId) -> Result<Vec<DiscussionItem>>;
    fn touch(&mut self, data: DiscussionId) -> Result<Executed>;
}

pub trait MessageStore {
    fn by_discussion_id(&mut self, discussion_id: &DiscussionId) -> Result<Vec<Message>>;
    fn create(&mut self, data: &Message) -> Result<Message>;
    fn create_many(&mut self, data: &[Message]) -> Result<Vec<Message>>;
}

pub trait InviteStore {
    fn by_token(&mut self, token: &InviteToken) -> Result<Invite>;
    fn create(&mut self, data: &Invite) -> Result<Invite>;
    fn update(&mut self, data: &Invite) -> Result<Invite>;
}

pub trait WorkflowableStore {
    fn create(&mut self, data: &Workflowable) -> Result<Workflowable>;
}

pub trait WorkflowStore {
    fn by_workflowable_id(
        &mut self,
        workflowable_id: &WorkflowableId,
    ) -> Result<Option<WorkflowWithSteps>>;
    fn create(&mut self, data: &Workflow) -> Result<Workflow>;
}

pub trait StepStore {
    fn by_id(&mut self, id: &StepId) -> Result<Step>;
    fn by_workflow_id(&mut self, workflow_id: &WorkflowId) -> Result<Vec<Step>>;
    fn create(&mut self, data: &Step) -> Result<Step>;
    fn create_many(&mut self, data: &[Step]) -> Result<Vec<Step>>;
    fn update(&mut self, data: &Step) -> Result<Step>;
}
