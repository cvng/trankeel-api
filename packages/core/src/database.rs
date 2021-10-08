use crate::error::Error;
use piteo_data::Account;
use piteo_data::AccountData;
use piteo_data::AccountId;
use piteo_data::Advertisement;
use piteo_data::AdvertisementData;
use piteo_data::AdvertisementId;
use piteo_data::AuthId;
use piteo_data::Candidacy;
use piteo_data::CandidacyData;
use piteo_data::CandidacyId;
use piteo_data::Company;
use piteo_data::CompanyId;
use piteo_data::Discussion;
use piteo_data::DiscussionId;
use piteo_data::DiscussionSubject;
use piteo_data::Event;
use piteo_data::EventId;
use piteo_data::EventWithEventable;
use piteo_data::ExternalId;
use piteo_data::File;
use piteo_data::FileData;
use piteo_data::FileId;
use piteo_data::Id;
use piteo_data::Lease;
use piteo_data::LeaseData;
use piteo_data::LeaseId;
use piteo_data::Lender;
use piteo_data::LenderData;
use piteo_data::LenderId;
use piteo_data::LenderWithIdentity;
use piteo_data::Message;
use piteo_data::Payment;
use piteo_data::PaymentNoticeId;
use piteo_data::Person;
use piteo_data::PersonData;
use piteo_data::PersonId;
use piteo_data::Plan;
use piteo_data::PlanId;
use piteo_data::Property;
use piteo_data::PropertyData;
use piteo_data::PropertyId;
use piteo_data::ReceiptId;
use piteo_data::Rent;
use piteo_data::RentData;
use piteo_data::RentId;
use piteo_data::Summary;
use piteo_data::Tenant;
use piteo_data::TenantData;
use piteo_data::TenantId;
use piteo_data::Warrant;
use piteo_data::WarrantId;
use piteo_data::WarrantWithIdentity;

pub type Result<T> = std::result::Result<T, Error>;

pub type Executed = usize;

pub trait Db {
    fn accounts(&self) -> Box<dyn AccountStore + '_>;
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
    fn reports(&self) -> Box<dyn ReportStore + '_>;
    fn discussions(&self) -> Box<dyn DiscussionStore + '_>;
    fn messages(&self) -> Box<dyn MessageStore + '_>;
}

pub trait AccountStore {
    fn by_id(&mut self, id: &AccountId) -> Result<Account>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Account>;
    fn by_advertisement_id(&mut self, advertisement_id: &AdvertisementId) -> Result<Account>;
    fn by_person_id(&mut self, person_id: &PersonId) -> Result<Account>;
    fn create(&mut self, data: Account) -> Result<Account>;
    fn update(&mut self, data: AccountData) -> Result<Account>;
}

pub trait PersonStore {
    fn by_id(&mut self, id: &PersonId) -> Result<Person>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Person>;
    fn by_account_id(&mut self, account_id: &AccountId) -> Result<Vec<Person>>;
    fn by_candidacy_id(&mut self, candidacy_id: &CandidacyId) -> Result<Person>;
    fn create(&mut self, data: Person) -> Result<Person>;
    fn update(&mut self, data: PersonData) -> Result<Person>;
}

pub trait CompanyStore {
    fn by_id(&mut self, id: &CompanyId) -> Result<Company>;
}

pub trait LenderStore {
    fn by_id(&mut self, id: &LenderId) -> Result<LenderWithIdentity>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<LenderWithIdentity>>;
    fn by_individual_id(&mut self, individual_id: &PersonId) -> Result<LenderWithIdentity>;
    fn create(&mut self, data: Lender) -> Result<Lender>;
    fn update(&mut self, data: LenderData) -> Result<Lender>;
}

pub trait AdvertisementStore {
    fn by_id(&mut self, id: &AdvertisementId) -> Result<Advertisement>;
    fn by_candidacy_id(&mut self, candidacy_id: &CandidacyId) -> Result<Advertisement>;
    fn by_property_id(&mut self, property_id: &PropertyId) -> Result<Vec<Advertisement>>;
    fn create(&mut self, data: Advertisement) -> Result<Advertisement>;
    fn update(&mut self, data: AdvertisementData) -> Result<Advertisement>;
}

pub trait CandidacyStore {
    fn by_id(&mut self, id: &CandidacyId) -> Result<Candidacy>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Candidacy>>;
    fn by_property_id(&mut self, property_id: &PropertyId) -> Result<Vec<Candidacy>>;
    fn create(&mut self, data: Candidacy) -> Result<Candidacy>;
    fn update(&mut self, data: CandidacyData) -> Result<Candidacy>;
    fn update_by_advertisement_id(
        &mut self,
        advertisement_id: &AdvertisementId,
        data: CandidacyData,
    ) -> Result<Vec<Candidacy>>;
}

pub trait PropertyStore {
    fn by_id(&mut self, id: &PropertyId) -> Result<Property>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Property>>;
    fn by_advertisement_id(&mut self, advertisement_id: &AdvertisementId) -> Result<Property>;
    fn create(&mut self, data: Property) -> Result<Property>;
    fn delete(&mut self, data: PropertyId) -> Result<Executed>;
    fn update(&mut self, data: PropertyData) -> Result<Property>;
}

pub trait TenantStore {
    fn by_id(&mut self, id: &TenantId) -> Result<Tenant>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Tenant>>;
    fn by_lease_id(&mut self, lease_id: &LeaseId) -> Result<Vec<Tenant>>;
    fn create(&mut self, data: Tenant) -> Result<Tenant>;
    fn delete(&mut self, data: TenantId) -> Result<Executed>;
    fn update(&mut self, data: TenantData) -> Result<Tenant>;
}

pub trait WarrantStore {
    fn by_id(&mut self, id: &WarrantId) -> Result<WarrantWithIdentity>;
    fn by_tenant_id(&mut self, tenant_id: &TenantId) -> Result<Vec<WarrantWithIdentity>>;
    fn create(&mut self, data: WarrantWithIdentity) -> Result<WarrantWithIdentity>;
    fn with_identity(&mut self, data: Warrant) -> Result<WarrantWithIdentity>;
}

pub trait LeaseStore {
    fn by_id(&mut self, id: &LeaseId) -> Result<Lease>;
    fn by_property_id(&mut self, property_id: &PropertyId) -> Result<Vec<Lease>>;
    fn by_receipt_id(&mut self, receipt_id: &ReceiptId) -> Result<Lease>;
    fn by_notice_id(&mut self, notice_id: &PaymentNoticeId) -> Result<Lease>;
    fn by_rent_id(&mut self, rent_id: &RentId) -> Result<Lease>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Lease>>;
    fn create(&mut self, data: Lease) -> Result<Lease>;
    fn delete(&mut self, data: LeaseId) -> Result<Executed>;
    fn update(&mut self, data: LeaseData) -> Result<Lease>;
}

pub trait RentStore {
    fn by_id(&mut self, id: &RentId) -> Result<Rent>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Rent>>;
    fn by_receipt_id(&mut self, receipt_id: &ReceiptId) -> Result<Rent>;
    fn by_notice_id(&mut self, notice_id: &PaymentNoticeId) -> Result<Rent>;
    fn by_lease_id(&mut self, lease_id: &LeaseId) -> Result<Vec<Rent>>;
    fn create(&mut self, data: Rent) -> Result<Rent>;
    fn create_many(&mut self, data: Vec<Rent>) -> Result<Vec<Rent>>;
    fn update(&mut self, data: RentData) -> Result<Rent>;
}

pub trait FileStore {
    fn by_id(&mut self, id: &FileId) -> Result<File>;
    #[allow(clippy::ptr_arg)]
    fn by_external_id(&mut self, external_id: &ExternalId) -> Result<File>;
    fn create(&mut self, data: File) -> Result<File>;
    fn update(&mut self, data: FileData) -> Result<File>;
}

pub trait PaymentStore {
    fn create(&mut self, data: Payment) -> Result<Payment>;
    fn by_rent_id(&mut self, rent_id: &RentId) -> Result<Vec<Payment>>;
}

pub trait PlanStore {
    fn by_id(&mut self, id: &PlanId) -> Result<Plan>;
}

pub trait EventStore {
    fn by_id(&mut self, id: &EventId) -> Result<EventWithEventable>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<EventWithEventable>>;
    fn create(&mut self, data: Event) -> Result<Event>;
}

pub trait ReportStore {
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Summary>;
}

pub trait DiscussionStore {
    fn by_id(&mut self, id: &DiscussionId) -> Result<Discussion>;
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Discussion>>;
    fn by_subject_id(&mut self, subject_id: &Id) -> Result<Discussion>;
    fn create(&mut self, data: Discussion) -> Result<Discussion>;
    fn delete(&mut self, data: DiscussionId) -> Result<Executed>;
    fn related_subject(&mut self, id: &DiscussionId) -> Result<Option<DiscussionSubject>>;
    fn touch(&mut self, data: DiscussionId) -> Result<Executed>;
}

pub trait MessageStore {
    fn by_discussion_id(&mut self, discussion_id: &DiscussionId) -> Result<Vec<Message>>;
    fn create(&mut self, data: Message) -> Result<Message>;
}
