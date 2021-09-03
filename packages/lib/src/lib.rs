use piteo_core::auth;
use piteo_core::database::Conn;
use piteo_core::database::Db;
use piteo_core::database::TenantStore;
use piteo_core::diesel::insert_into;
use piteo_core::diesel::prelude::*;
use piteo_core::error::Error;
use piteo_core::schema::tenant;
use piteo_core::schema::user;
use piteo_core::tenants;
use piteo_core::tenants::ops::TenantInput;
use piteo_core::AuthId;
use piteo_core::Tenant;
use piteo_core::TenantId;

pub struct Database<'a>(&'a Conn);

pub struct DatabaseTenantStore<'a>(&'a Conn);

impl<'a> Database<'a> {
    pub fn new(conn: &'a Conn) -> Self {
        Self(conn)
    }
}

impl<'a> Db<'a> for Database<'a> {
    fn tenants(&self) -> Box<dyn TenantStore + 'a> {
        Box::new(DatabaseTenantStore(self.0))
    }
}

impl<'a> DatabaseTenantStore<'a> {
    pub fn conn(&self) -> &'a Conn {
        self.0
    }
}

impl TenantStore for DatabaseTenantStore<'_> {
    fn all(&mut self, auth_id: AuthId, id: Option<TenantId>) -> Result<Vec<Tenant>, Error> {
        let query = tenant::table
            .select(tenant::all_columns)
            .left_join(user::table.on(user::accountId.eq(tenant::account_id.nullable())))
            .filter(user::authId.eq(auth_id.inner()));

        match id {
            Some(id) => query
                .filter(tenant::id.eq(id))
                .load(self.conn())
                .map_err(|err| err.into()),
            None => query.load(self.conn()).map_err(|err| err.into()),
        }
    }

    fn insert(&mut self, auth_id: AuthId, data: TenantInput) -> Result<Tenant, Error> {
        let account = auth::get_account_by_auth_id(self.conn(), auth_id)?;

        Ok(insert_into(tenant::table)
            .values((
                tenant::account_id.eq(account.id),
                tenant::birthdate.eq(data.birthdate),
                tenant::email.eq(data.email),
                tenant::first_name.eq(data.first_name),
                tenant::last_name.eq(data.last_name),
            ))
            .get_result(self.conn())?)
    }
}

pub fn all_tenants(
    conn: &Conn,
    auth_id: AuthId,
    id: Option<TenantId>,
) -> Result<Vec<Tenant>, Error> {
    tenants::all_tenants(Database(conn), auth_id, id)
}

pub fn create_tenant(conn: &Conn, auth_id: AuthId, data: TenantInput) -> Result<Tenant, Error> {
    tenants::ops::create_tenant(Database::new(conn), auth_id, data)
}
