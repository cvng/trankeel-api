use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::update;
use diesel::PgConnection;
use piteo_core::database::AccountStore;
use piteo_core::database::Db;
use piteo_core::database::LeaseStore;
use piteo_core::database::LeaseTenantStore;
use piteo_core::database::LenderStore;
use piteo_core::database::PropertyStore;
use piteo_core::database::RentStore;
use piteo_core::database::TenantStore;
use piteo_core::database::UserStore;
use piteo_core::error::Context;
use piteo_core::error::Error;
use piteo_core::schema::account;
use piteo_core::schema::lease;
use piteo_core::schema::leasetenant;
use piteo_core::schema::lender;
use piteo_core::schema::property;
use piteo_core::schema::rent;
use piteo_core::schema::tenant;
use piteo_core::schema::user;
use piteo_core::Account;
use piteo_core::AccountData;
use piteo_core::AuthId;
use piteo_core::Lease;
use piteo_core::LeaseData;
use piteo_core::LeaseId;
use piteo_core::LeaseTenant;
use piteo_core::Lender;
use piteo_core::LenderData;
use piteo_core::LenderId;
use piteo_core::Person;
use piteo_core::PersonData;
use piteo_core::Property;
use piteo_core::PropertyData;
use piteo_core::PropertyId;
use piteo_core::Rent;
use piteo_core::Tenant;
use piteo_core::TenantData;
use piteo_core::TenantId;

type Result<T> = std::result::Result<T, Error>;

type Deleted = usize;

/// Database pool.
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Build connection pool.
pub fn build_connection_pool(database_url: &str) -> Result<DbPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .context(format!("Error connecting to {}", database_url))
}

pub struct Database(DbPool);

pub struct DatabaseAccountStore<'a>(&'a DbPool);

pub struct DatabaseUserStore<'a>(&'a DbPool);

pub struct DatabaseTenantStore<'a>(&'a DbPool);

pub struct DatabaseLenderStore<'a>(&'a DbPool);

pub struct DatabasePropertyStore<'a>(&'a DbPool);

pub struct DatabaseLeaseStore<'a>(&'a DbPool);

pub struct DatabaseLeaseTenantStore<'a>(&'a DbPool);

pub struct DatabaseRentStore<'a>(&'a DbPool);

impl Database {
    pub fn new(pool: DbPool) -> Self {
        Self(pool)
    }
}

impl Db for Database {
    fn accounts(&self) -> Box<dyn AccountStore + '_> {
        Box::new(DatabaseAccountStore(&self.0))
    }

    fn users(&self) -> Box<dyn UserStore + '_> {
        Box::new(DatabaseUserStore(&self.0))
    }

    fn lenders(&self) -> Box<dyn LenderStore + '_> {
        Box::new(DatabaseLenderStore(&self.0))
    }

    fn tenants(&self) -> Box<dyn TenantStore + '_> {
        Box::new(DatabaseTenantStore(&self.0))
    }

    fn properties(&self) -> Box<dyn PropertyStore + '_> {
        Box::new(DatabasePropertyStore(&self.0))
    }

    fn leases(&self) -> Box<dyn LeaseStore + '_> {
        Box::new(DatabaseLeaseStore(&self.0))
    }

    fn lease_tenants(&self) -> Box<dyn LeaseTenantStore + '_> {
        Box::new(DatabaseLeaseTenantStore(&self.0))
    }

    fn rents(&self) -> Box<dyn RentStore + '_> {
        Box::new(DatabaseRentStore(&self.0))
    }
}

impl AccountStore for DatabaseAccountStore<'_> {
    fn by_auth_id(&mut self, auth_id: AuthId) -> Result<Account> {
        account::table
            .select(account::all_columns)
            .left_join(user::table.on(user::account_id.eq(account::id.nullable())))
            .filter(user::auth_id.eq(&auth_id.inner()))
            .first(&self.0.get()?)
            .map_err(|err| err.into())
    }

    fn create(&mut self, data: Account) -> Result<Account> {
        Ok(insert_into(account::table)
            .values((
                account::plan_id.eq(data.plan_id),
                account::status.eq(data.status),
                account::stripe_customer_id.eq(data.stripe_customer_id),
                account::stripe_subscription_id.eq(data.stripe_subscription_id),
                account::trial_end.eq(data.trial_end),
                account::owner_id.eq(data.owner_id),
            ))
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: AccountData) -> Result<Account> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }
}

impl UserStore for DatabaseUserStore<'_> {
    fn create(&mut self, data: Person) -> Result<Person> {
        Ok(insert_into(user::table)
            .values((
                user::auth_id.eq(data.auth_id),
                user::email.eq(data.email),
                user::first_name.eq(data.first_name),
                user::last_name.eq(data.last_name),
                user::address.eq(data.address),
                user::photo_url.eq(data.photo_url),
                user::role.eq(data.role),
                user::phone_number.eq(data.phone_number),
                user::account_id.eq(data.account_id),
            ))
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: PersonData) -> Result<Person> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }
}

impl TenantStore for DatabaseTenantStore<'_> {
    fn all(&mut self, auth_id: AuthId, id: Option<TenantId>) -> Result<Vec<Tenant>> {
        let query = tenant::table
            .select(tenant::all_columns)
            .left_join(user::table.on(user::account_id.eq(tenant::account_id.nullable())))
            .filter(user::auth_id.eq(auth_id.inner()));

        match id {
            Some(id) => query
                .filter(tenant::id.eq(id))
                .load(&self.0.get()?)
                .map_err(|err| err.into()),
            None => query.load(&self.0.get()?).map_err(|err| err.into()),
        }
    }

    fn create(&mut self, data: Tenant) -> Result<Tenant> {
        Ok(insert_into(tenant::table)
            .values((
                tenant::account_id.eq(data.account_id),
                tenant::apl.eq(data.apl),
                tenant::auth_id.eq(data.auth_id),
                tenant::birthdate.eq(data.birthdate),
                tenant::birthplace.eq(data.birthplace),
                tenant::email.eq(data.email),
                tenant::first_name.eq(data.first_name),
                tenant::last_name.eq(data.last_name),
                tenant::note.eq(data.note),
                tenant::phone_number.eq(data.phone_number),
                tenant::role.eq(data.role),
                tenant::lease_id.eq(data.lease_id),
                tenant::visale_id.eq(data.visale_id),
            ))
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: TenantData) -> Result<Tenant> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }

    fn delete(&mut self, data: TenantId) -> Result<Deleted> {
        Ok(delete(tenant::table)
            .filter(tenant::id.eq(data))
            .execute(&self.0.get()?)?)
    }
}

impl LenderStore for DatabaseLenderStore<'_> {
    fn by_id(&mut self, id: LenderId) -> Result<Lender> {
        lender::table
            .find(id)
            .first(&self.0.get()?)
            .map_err(|err| err.into())
    }

    fn create(&mut self, data: Lender) -> Result<Lender> {
        Ok(insert_into(lender::table)
            .values((
                lender::account_id.eq(data.account_id),
                lender::individual_id.eq(data.individual_id),
                lender::company_id.eq(data.company_id),
            ))
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: LenderData) -> Result<Lender> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }
}

impl PropertyStore for DatabasePropertyStore<'_> {
    fn all(&mut self, auth_id: AuthId, id: Option<PropertyId>) -> Result<Vec<Property>> {
        let query = property::table
            .select(property::all_columns)
            .left_join(user::table.on(user::account_id.eq(property::account_id)))
            .filter(user::auth_id.eq(auth_id.inner()));

        if let Some(id) = id {
            return query
                .filter(property::id.eq(id))
                .load(&self.0.get()?)
                .map_err(|err| err.into());
        }

        query.load(&self.0.get()?).map_err(|err| err.into())
    }

    fn create(&mut self, data: Property) -> Result<Property> {
        Ok(insert_into(property::table)
            .values((
                property::account_id.eq(data.account_id),
                property::address.eq(data.address),
                property::build_period.eq(data.build_period),
                property::building_legal_status.eq(data.building_legal_status),
                property::common_spaces.eq(data.common_spaces),
                property::energy_class.eq(data.energy_class),
                property::equipments.eq(data.equipments),
                property::gas_emission.eq(data.gas_emission),
                property::heating_method.eq(data.heating_method),
                property::housing_type.eq(data.housing_type),
                property::name.eq(data.name),
                property::note.eq(data.note),
                property::ntic_equipments.eq(data.ntic_equipments),
                property::other_spaces.eq(data.other_spaces),
                property::tax.eq(data.tax),
                property::room_count.eq(data.room_count),
                property::status.eq(data.status),
                property::surface.eq(data.surface),
                property::tenant_private_spaces.eq(data.tenant_private_spaces),
                property::usage_type.eq(data.usage_type),
                property::water_heating_method.eq(data.water_heating_method),
                property::lender_id.eq(data.lender_id),
            ))
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: PropertyData) -> Result<Property> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }

    fn delete(&mut self, data: PropertyId) -> Result<Deleted> {
        Ok(delete(property::table)
            .filter(property::id.eq(data))
            .execute(&self.0.get()?)?)
    }
}

impl LeaseStore for DatabaseLeaseStore<'_> {
    fn create(&mut self, data: Lease) -> Result<Lease> {
        Ok(insert_into(lease::table)
            .values((
                lease::account_id.eq(data.account_id),
                lease::deposit_amount.eq(data.deposit_amount),
                lease::effect_date.eq(data.effect_date),
                lease::signature_date.eq(data.signature_date),
                lease::rent_amount.eq(data.rent_amount),
                lease::rent_charges_amount.eq(data.rent_charges_amount),
                lease::type_.eq(data.type_),
                lease::lease_id.eq(data.lease_id),
                lease::property_id.eq(data.property_id),
                lease::details.eq(data.details),
                lease::expired_at.eq(data.expired_at),
                lease::renew_date.eq(data.renew_date),
                lease::duration.eq(data.duration),
            ))
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: LeaseData) -> Result<Lease> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }

    fn delete(&mut self, data: LeaseId) -> Result<Deleted> {
        Ok(delete(lease::table)
            .filter(lease::id.eq(data))
            .execute(&self.0.get()?)?)
    }
}

impl LeaseTenantStore for DatabaseLeaseTenantStore<'_> {
    fn create(&mut self, data: LeaseTenant) -> Result<LeaseTenant> {
        Ok(insert_into(leasetenant::table)
            .values((
                leasetenant::lease_id.eq(data.lease_id),
                leasetenant::tenant_id.eq(data.tenant_id),
            ))
            .get_result(&self.0.get()?)?)
    }

    fn create_many(&mut self, data: Vec<LeaseTenant>) -> Result<Vec<LeaseTenant>> {
        data.iter().map(|item| self.create(item.clone())).collect()
    }
}

impl RentStore for DatabaseRentStore<'_> {
    fn create(&mut self, data: Rent) -> Result<Rent> {
        Ok(insert_into(rent::table)
            .values((
                rent::period_end.eq(data.period_end),
                rent::period_start.eq(data.period_start),
                rent::amount.eq(data.amount),
                rent::charges_amount.eq(data.charges_amount),
                rent::full_amount.eq(data.full_amount),
                rent::status.eq(data.status),
                rent::lease_id.eq(data.lease_id),
                rent::receipt_id.eq(data.receipt_id),
                rent::transaction_id.eq(data.transaction_id),
                rent::notice_id.eq(data.notice_id),
            ))
            .get_result(&self.0.get()?)?)
    }

    fn create_many(&mut self, data: Vec<Rent>) -> Result<Vec<Rent>> {
        data.iter().map(|item| self.create(item.clone())).collect()
    }
}
