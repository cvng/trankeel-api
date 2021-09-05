use eyre::Error;
use piteo_data::Account;
use piteo_data::AccountData;
use piteo_data::AuthId;
use piteo_data::Lender;
use piteo_data::LenderData;
use piteo_data::Person;
use piteo_data::PersonData;
use piteo_data::Property;
use piteo_data::PropertyData;
use piteo_data::PropertyId;
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
}

pub trait AccountStore {
    fn by_auth_id(&mut self, auth_id: AuthId) -> Result<Account>;
    fn create(&mut self, data: AccountData) -> Result<Account>;
    fn update(&mut self, data: Account) -> Result<Account>;
}

pub trait UserStore {
    fn create(&mut self, data: PersonData) -> Result<Person>;
    fn update(&mut self, data: Person) -> Result<Person>;
}

pub trait LenderStore {
    fn create(&mut self, data: LenderData) -> Result<Lender>;
}

pub trait PropertyStore {
    fn create(&mut self, data: Property) -> Result<Property>;
    fn update(&mut self, data: PropertyData) -> Result<Property>;
    fn delete(&mut self, data: PropertyId) -> Result<Deleted>;
}

pub trait TenantStore {
    fn all(&mut self, auth_id: AuthId, id: Option<TenantId>) -> Result<Vec<Tenant>>;
    fn create(&mut self, data: Tenant) -> Result<Tenant>;
    fn update(&mut self, data: TenantData) -> Result<Tenant>;
    fn delete(&mut self, data: TenantId) -> Result<Deleted>;
}
