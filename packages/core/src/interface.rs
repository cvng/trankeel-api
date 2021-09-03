use crate::tenants::ops::TenantInput;
use eyre::Error;
use piteo_data::AuthId;
use piteo_data::Tenant;
use piteo_data::TenantId;
use std::collections::BTreeMap;

pub trait Db<'a> {
    fn tenants(&self) -> Box<dyn TenantStore + 'a>;
}

pub trait TenantStore {
    fn all(&mut self, auth_id: AuthId, id: Option<TenantId>) -> Result<Vec<Tenant>, Error>;

    fn insert(&mut self, auth_id: AuthId, data: TenantInput) -> Result<Tenant, Error>;
}

pub struct InMemoryDb;

pub struct InMemoryTenantStore(BTreeMap<TenantId, Tenant>);

impl Db<'_> for InMemoryDb {
    fn tenants(&self) -> Box<dyn TenantStore> {
        Box::new(InMemoryTenantStore(BTreeMap::new()))
    }
}

impl TenantStore for InMemoryTenantStore {
    fn all(&mut self, _auth_id: AuthId, _id: Option<TenantId>) -> Result<Vec<Tenant>, Error> {
        todo!()
    }

    fn insert(&mut self, auth_id: AuthId, data: TenantInput) -> Result<Tenant, Error> {
        let id = TenantId::new_v4();
        let account_id = TenantId::new_v4();
        self.0.insert(
            id,
            Tenant {
                id,
                account_id,
                auth_id: Some(auth_id),
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