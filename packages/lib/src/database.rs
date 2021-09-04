use crate::Conn;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::update;
use piteo_core::database::AccountStore;
use piteo_core::database::Db;
use piteo_core::database::LenderStore;
use piteo_core::database::TenantStore;
use piteo_core::database::UserStore;
use piteo_core::error::Error;
use piteo_core::schema::account;
use piteo_core::schema::lender;
use piteo_core::schema::tenant;
use piteo_core::schema::user;
use piteo_core::Account;
use piteo_core::AccountData;
use piteo_core::AuthId;
use piteo_core::Lender;
use piteo_core::LenderData;
use piteo_core::Person;
use piteo_core::PersonData;
use piteo_core::Tenant;
use piteo_core::TenantData;
use piteo_core::TenantId;

pub struct Database<'a>(&'a Conn);

pub struct DatabaseAccountStore<'a>(&'a Conn);

pub struct DatabaseUserStore<'a>(&'a Conn);

pub struct DatabaseTenantStore<'a>(&'a Conn);

pub struct DatabaseLenderStore<'a>(&'a Conn);

impl<'a> Database<'a> {
    pub fn new(conn: &'a Conn) -> Self {
        Self(conn)
    }
}

impl<'a> Db<'a> for Database<'a> {
    fn accounts(&self) -> Box<dyn AccountStore + 'a> {
        Box::new(DatabaseAccountStore(self.0))
    }

    fn users(&self) -> Box<dyn UserStore + 'a> {
        Box::new(DatabaseUserStore(self.0))
    }

    fn tenants(&self) -> Box<dyn TenantStore + 'a> {
        Box::new(DatabaseTenantStore(self.0))
    }

    fn lenders(&self) -> Box<dyn piteo_core::database::LenderStore + 'a> {
        Box::new(DatabaseLenderStore(self.0))
    }
}

impl AccountStore for DatabaseAccountStore<'_> {
    fn by_auth_id(&mut self, auth_id: AuthId) -> Result<Account, Error> {
        account::table
            .select(account::all_columns)
            .left_join(user::table.on(user::account_id.eq(account::id.nullable())))
            .filter(user::auth_id.eq(&auth_id.inner()))
            .first(self.0)
            .map_err(|err| err.into())
    }

    fn create(&mut self, data: AccountData) -> Result<Account, Error> {
        Ok(insert_into(account::table)
            .values(data)
            .get_result(self.0)?)
    }
}

impl UserStore for DatabaseUserStore<'_> {
    fn create(&mut self, data: PersonData) -> Result<Person, Error> {
        Ok(insert_into(user::table).values(data).get_result(self.0)?)
    }

    fn update(&mut self, data: Person) -> Result<Person, Error> {
        Ok(update(&data).set(&data).get_result(self.0)?)
    }
}

impl TenantStore for DatabaseTenantStore<'_> {
    fn all(&mut self, auth_id: AuthId, id: Option<TenantId>) -> Result<Vec<Tenant>, Error> {
        let query = tenant::table
            .select(tenant::all_columns)
            .left_join(user::table.on(user::account_id.eq(tenant::account_id.nullable())))
            .filter(user::auth_id.eq(auth_id.inner()));

        match id {
            Some(id) => query
                .filter(tenant::id.eq(id))
                .load(self.0)
                .map_err(|err| err.into()),
            None => query.load(self.0).map_err(|err| err.into()),
        }
    }

    fn create(&mut self, data: TenantData) -> Result<Tenant, Error> {
        Ok(insert_into(tenant::table).values(data).get_result(self.0)?)
    }
}

impl LenderStore for DatabaseLenderStore<'_> {
    fn create(&mut self, data: LenderData) -> Result<Lender, Error> {
        Ok(insert_into(lender::table).values(data).get_result(self.0)?)
    }
}
