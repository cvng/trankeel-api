use eyre::Error;
use piteo_data::Account;
use piteo_data::AccountData;
use piteo_data::AuthId;
use piteo_data::Lender;
use piteo_data::LenderData;
use piteo_data::Person;
use piteo_data::PersonData;
use piteo_data::Tenant;
use piteo_data::TenantData;
use piteo_data::TenantId;

// # Interfaces

pub trait Db {
    fn accounts(&self) -> Box<dyn AccountStore + '_>;
    fn users(&self) -> Box<dyn UserStore + '_>;
    fn lenders(&self) -> Box<dyn LenderStore + '_>;
    fn tenants(&self) -> Box<dyn TenantStore + '_>;
}

pub trait AccountStore {
    fn by_auth_id(&mut self, auth_id: AuthId) -> Result<Account, Error>;
    fn create(&mut self, data: AccountData) -> Result<Account, Error>;
    fn update(&mut self, data: Account) -> Result<Account, Error>;
}

pub trait UserStore {
    fn create(&mut self, data: PersonData) -> Result<Person, Error>;
    fn update(&mut self, data: Person) -> Result<Person, Error>;
}

pub trait LenderStore {
    fn create(&mut self, data: LenderData) -> Result<Lender, Error>;
}

pub trait TenantStore {
    fn all(&mut self, auth_id: AuthId, id: Option<TenantId>) -> Result<Vec<Tenant>, Error>;
    fn create(&mut self, data: Tenant) -> Result<Tenant, Error>;
    fn update(&mut self, data: TenantData) -> Result<Tenant, Error>;
    fn delete(&mut self, data: TenantId) -> Result<usize, Error>;
}
