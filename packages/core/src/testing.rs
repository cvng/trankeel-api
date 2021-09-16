use crate::database::AccountStore;
use crate::database::Db;
use crate::database::LenderStore;
use crate::database::PropertyStore;
use crate::database::TenantStore;
use crate::database::UserStore;
use eyre::Error;
use piteo_data::Account;
use piteo_data::AccountData;
use piteo_data::AccountId;
use piteo_data::AuthId;
use piteo_data::LeaseId;
use piteo_data::Tenant;
use piteo_data::TenantData;
use piteo_data::TenantId;
use std::collections::BTreeMap;

#[allow(dead_code)]
pub fn db() -> InMemoryDb {
    InMemoryDb::new()
}

pub struct InMemoryDb;

pub struct InMemoryAccountStore(BTreeMap<AccountId, Account>);

pub struct InMemoryTenantStore(BTreeMap<TenantId, Tenant>);

impl InMemoryDb {
    pub fn new() -> Self {
        Self
    }
}

impl Default for InMemoryDb {
    fn default() -> Self {
        Self::new()
    }
}

impl Db for InMemoryDb {
    fn accounts(&self) -> Box<dyn AccountStore> {
        Box::new(InMemoryAccountStore(BTreeMap::new()))
    }

    fn users(&self) -> Box<dyn UserStore> {
        todo!()
    }

    fn lenders(&self) -> Box<dyn LenderStore> {
        todo!()
    }

    fn tenants(&self) -> Box<dyn TenantStore> {
        Box::new(InMemoryTenantStore(BTreeMap::new()))
    }

    fn properties(&self) -> Box<dyn PropertyStore + '_> {
        todo!()
    }

    fn leases(&self) -> Box<dyn crate::database::LeaseStore + '_> {
        todo!()
    }

    fn rents(&self) -> Box<dyn crate::database::RentStore + '_> {
        todo!()
    }

    fn files(&self) -> Box<dyn crate::database::FileStore + '_> {
        todo!()
    }
}

impl AccountStore for InMemoryAccountStore {
    fn by_auth_id(&mut self, _auth_id: AuthId) -> Result<Account, Error> {
        Ok(Account {
            id: Default::default(),
            created_at: Default::default(),
            updated_at: Default::default(),
            plan_id: Default::default(),
            status: Default::default(),
            stripe_customer_id: Default::default(),
            stripe_subscription_id: Default::default(),
            trial_end: Default::default(),
        })
    }

    fn create(&mut self, _data: Account) -> Result<Account, Error> {
        todo!()
    }

    fn update(&mut self, _data: AccountData) -> Result<Account, Error> {
        todo!()
    }
}

impl TenantStore for InMemoryTenantStore {
    fn all(&mut self, _auth_id: AuthId, _id: Option<TenantId>) -> Result<Vec<Tenant>, Error> {
        Ok(self.0.clone().into_values().collect::<Vec<Tenant>>())
    }

    fn by_lease_id(&mut self, _id: LeaseId) -> Result<Vec<Tenant>, Error> {
        todo!()
    }

    fn create(&mut self, data: Tenant) -> Result<Tenant, Error> {
        let id = TenantId::new_v4();
        Ok(self.0.entry(id).or_insert(Tenant { id, ..data }).clone())
    }

    fn delete(&mut self, _data: TenantId) -> Result<usize, Error> {
        todo!()
    }

    fn update(&mut self, _data: TenantData) -> Result<Tenant, Error> {
        todo!()
    }
}
