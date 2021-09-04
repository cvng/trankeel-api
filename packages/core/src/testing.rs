use crate::database::AccountStore;
use crate::database::Db;
use crate::database::LenderStore;
use crate::database::TenantStore;
use crate::database::UserStore;
use eyre::Error;
use piteo_data::Account;
use piteo_data::AccountData;
use piteo_data::AccountId;
use piteo_data::AuthId;
use piteo_data::Tenant;
use piteo_data::TenantData;
use piteo_data::TenantId;
use std::collections::BTreeMap;

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

    fn tenants(&self) -> Box<dyn TenantStore> {
        Box::new(InMemoryTenantStore(BTreeMap::new()))
    }

    fn lenders(&self) -> Box<dyn LenderStore> {
        todo!()
    }
}

impl AccountStore for InMemoryAccountStore {
    fn by_auth_id(&mut self, _auth_id: AuthId) -> Result<Account, Error> {
        Ok(Account {
            plan_id: Default::default(),
            status: Default::default(),
            stripe_customer_id: Default::default(),
            stripe_subscription_id: Default::default(),
            trial_end: Default::default(),
            owner_id: Default::default(),
            id: Default::default(),
        })
    }

    fn create(&mut self, _data: AccountData) -> Result<Account, Error> {
        todo!()
    }

    fn update(&mut self, _data: Account) -> Result<Account, Error> {
        todo!()
    }
}

impl TenantStore for InMemoryTenantStore {
    fn all(&mut self, _auth_id: AuthId, _id: Option<TenantId>) -> Result<Vec<Tenant>, Error> {
        Ok(self.0.clone().into_values().collect::<Vec<Tenant>>())
    }

    fn create(&mut self, data: TenantData) -> Result<Tenant, Error> {
        let id = TenantId::new_v4();
        let account_id = TenantId::new_v4();
        self.0.insert(
            id,
            Tenant {
                id,
                account_id,
                auth_id: None,
                apl: data.apl.unwrap_or_default(),
                birthdate: data.birthdate,
                birthplace: data.birthplace,
                email: data.email,
                first_name: data.first_name,
                last_name: data.last_name,
                note: data.note,
                phone_number: data.phone_number,
                role: None,
                lease_id: None,
                visale_id: data.visale_id,
            },
        );
        Ok(self.0.get(&id).ok_or_else(|| Error::msg(""))?.clone())
    }
}
