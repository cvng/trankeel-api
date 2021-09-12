use eyre::Error;
use piteo_data::Account;
use piteo_data::AccountData;
use piteo_data::AuthId;
use piteo_data::ExternalId;
use piteo_data::File;
use piteo_data::FileData;
use piteo_data::FileId;
use piteo_data::Lease;
use piteo_data::LeaseData;
use piteo_data::LeaseId;
use piteo_data::LeaseTenant;
use piteo_data::Lender;
use piteo_data::LenderData;
use piteo_data::LenderId;
use piteo_data::LenderIdentity;
use piteo_data::Person;
use piteo_data::PersonData;
use piteo_data::Property;
use piteo_data::PropertyData;
use piteo_data::PropertyId;
use piteo_data::ReceiptId;
use piteo_data::Rent;
use piteo_data::RentData;
use piteo_data::RentId;
use piteo_data::Tenant;
use piteo_data::TenantData;
use piteo_data::TenantId;

type Result<T> = std::result::Result<T, Error>;

type Deleted = usize;

// # Interfaces

pub trait Db {
    fn accounts(&self) -> Box<dyn AccountStore + '_>;
    fn users(&self) -> Box<dyn UserStore + '_>;
    fn lenders(&self) -> Box<dyn LenderStore + '_>;
    fn tenants(&self) -> Box<dyn TenantStore + '_>;
    fn properties(&self) -> Box<dyn PropertyStore + '_>;
    fn leases(&self) -> Box<dyn LeaseStore + '_>;
    fn lease_tenants(&self) -> Box<dyn LeaseTenantStore + '_>;
    fn rents(&self) -> Box<dyn RentStore + '_>;
    fn files(&self) -> Box<dyn FileStore + '_>;
}

pub trait AccountStore {
    fn by_auth_id(&mut self, auth_id: AuthId) -> Result<Account>;
    fn create(&mut self, data: Account) -> Result<Account>;
    fn update(&mut self, data: AccountData) -> Result<Account>;
}

pub trait UserStore {
    fn create(&mut self, data: Person) -> Result<Person>;
    fn update(&mut self, data: PersonData) -> Result<Person>;
}

pub trait LenderStore {
    fn by_id(&mut self, id: LenderId) -> Result<LenderIdentity>;
    fn create(&mut self, data: Lender) -> Result<Lender>;
    fn update(&mut self, data: LenderData) -> Result<Lender>;
}

pub trait PropertyStore {
    fn all(&mut self, auth_id: AuthId, id: Option<PropertyId>) -> Result<Vec<Property>>;
    fn by_id(&mut self, id: PropertyId) -> Result<Property>;
    fn create(&mut self, data: Property) -> Result<Property>;
    fn delete(&mut self, data: PropertyId) -> Result<Deleted>;
    fn update(&mut self, data: PropertyData) -> Result<Property>;
}

pub trait TenantStore {
    fn all(&mut self, auth_id: AuthId, id: Option<TenantId>) -> Result<Vec<Tenant>>;
    fn by_lease_id(&mut self, id: LeaseId) -> Result<Vec<Tenant>>;
    fn create(&mut self, data: Tenant) -> Result<Tenant>;
    fn delete(&mut self, data: TenantId) -> Result<Deleted>;
    fn update(&mut self, data: TenantData) -> Result<Tenant>;
}

pub trait LeaseStore {
    fn by_id(&mut self, id: LeaseId) -> Result<Lease>;
    fn create(&mut self, data: Lease) -> Result<Lease>;
    fn delete(&mut self, data: LeaseId) -> Result<Deleted>;
    fn update(&mut self, data: LeaseData) -> Result<Lease>;
}

pub trait LeaseTenantStore {
    fn create(&mut self, data: LeaseTenant) -> Result<LeaseTenant>;
    fn create_many(&mut self, data: Vec<LeaseTenant>) -> Result<Vec<LeaseTenant>>;
}

pub trait RentStore {
    fn by_id(&mut self, id: &RentId) -> Result<Rent>;
    fn by_receipt_id(&mut self, receipt_id: ReceiptId) -> Result<Rent>;
    fn by_lease_id(&mut self, lease_id: LeaseId) -> Result<Vec<Rent>>;
    fn create(&mut self, data: &Rent) -> Result<Rent>;
    fn create_many(&mut self, data: Vec<Rent>) -> Result<Vec<Rent>>;
    fn update(&mut self, data: &RentData) -> Result<Rent>;
    fn update_many(&mut self, data: Vec<RentData>) -> Result<Vec<Rent>>;
}

pub trait FileStore {
    fn by_id(&mut self, id: FileId) -> Result<File>;
    fn by_external_id(&mut self, external_id: ExternalId) -> Result<File>;
    fn create(&mut self, data: &File) -> Result<File>;
    fn update(&mut self, data: &FileData) -> Result<File>;
}
