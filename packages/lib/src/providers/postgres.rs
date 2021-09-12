use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error::NotFound;
use diesel::update;
use diesel::PgConnection;
use piteo_core::database;
use piteo_core::database::Db;
use piteo_core::error::Context;
use piteo_core::error::Error;
use piteo_core::schema::account;
use piteo_core::schema::company;
use piteo_core::schema::file;
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
use piteo_core::ExternalId;
use piteo_core::File;
use piteo_core::FileData;
use piteo_core::FileId;
use piteo_core::Lease;
use piteo_core::LeaseData;
use piteo_core::LeaseId;
use piteo_core::LeaseTenant;
use piteo_core::Lender;
use piteo_core::LenderData;
use piteo_core::LenderId;
use piteo_core::LenderIdentity;
use piteo_core::Person;
use piteo_core::PersonData;
use piteo_core::Property;
use piteo_core::PropertyData;
use piteo_core::PropertyId;
use piteo_core::ReceiptId;
use piteo_core::Rent;
use piteo_core::RentData;
use piteo_core::RentId;
use piteo_core::Tenant;
use piteo_core::TenantData;
use piteo_core::TenantId;
use std::env;

type Result<T, E = Error> = std::result::Result<T, E>;

type Deleted = usize;

/// Database pool.
pub type DbPool = Pool<ConnectionManager<PgConnection>>;

/// Build database pool from env.
pub fn db_pool_from_env() -> Result<DbPool> {
    let database_url = env::var("DATABASE_URL").context("DATABASE_URL must be set.")?;

    build_connection_pool(&database_url)
}

/// Build connection pool.
pub fn build_connection_pool(database_url: &str) -> Result<DbPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .context(format!("Error connecting to {}", database_url))
}

/// Access database.
pub fn db(pool: DbPool) -> Pg {
    Pg::new(pool)
}

pub struct Pg(DbPool);

struct AccountStore<'a>(&'a DbPool);

struct UserStore<'a>(&'a DbPool);

struct TenantStore<'a>(&'a DbPool);

struct LenderStore<'a>(&'a DbPool);

struct PropertyStore<'a>(&'a DbPool);

struct LeaseStore<'a>(&'a DbPool);

struct LeaseTenantStore<'a>(&'a DbPool);

struct RentStore<'a>(&'a DbPool);

struct FileStore<'a>(&'a DbPool);

impl Pg {
    pub fn new(pool: DbPool) -> Self {
        Self(pool)
    }
}

impl Db for Pg {
    fn accounts(&self) -> Box<dyn database::AccountStore + '_> {
        Box::new(AccountStore(&self.0))
    }

    fn users(&self) -> Box<dyn database::UserStore + '_> {
        Box::new(UserStore(&self.0))
    }

    fn lenders(&self) -> Box<dyn database::LenderStore + '_> {
        Box::new(LenderStore(&self.0))
    }

    fn tenants(&self) -> Box<dyn database::TenantStore + '_> {
        Box::new(TenantStore(&self.0))
    }

    fn properties(&self) -> Box<dyn database::PropertyStore + '_> {
        Box::new(PropertyStore(&self.0))
    }

    fn leases(&self) -> Box<dyn database::LeaseStore + '_> {
        Box::new(LeaseStore(&self.0))
    }

    fn lease_tenants(&self) -> Box<dyn database::LeaseTenantStore + '_> {
        Box::new(LeaseTenantStore(&self.0))
    }

    fn rents(&self) -> Box<dyn database::RentStore + '_> {
        Box::new(RentStore(&self.0))
    }

    fn files(&self) -> Box<dyn database::FileStore + '_> {
        Box::new(FileStore(&self.0))
    }
}

impl database::AccountStore for AccountStore<'_> {
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

impl database::UserStore for UserStore<'_> {
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

impl database::TenantStore for TenantStore<'_> {
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

    fn by_lease_id(&mut self, lease_id: LeaseId) -> Result<Vec<Tenant>> {
        tenant::table
            .select(tenant::all_columns)
            .left_join(leasetenant::table.on(leasetenant::tenant_id.eq(tenant::id)))
            .filter(leasetenant::lease_id.eq(lease_id))
            .load(&self.0.get()?)
            .map_err(|err| err.into())
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

    fn delete(&mut self, data: TenantId) -> Result<Deleted> {
        Ok(delete(tenant::table)
            .filter(tenant::id.eq(data))
            .execute(&self.0.get()?)?)
    }

    fn update(&mut self, data: TenantData) -> Result<Tenant> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }
}

impl database::LenderStore for LenderStore<'_> {
    fn by_id(&mut self, id: LenderId) -> Result<LenderIdentity> {
        let lender_: Lender = lender::table.find(id).first(&self.0.get()?)?;

        match lender_ {
            Lender {
                individual_id: Some(individual_id),
                ..
            } => {
                let person = user::table.find(individual_id).first(&self.0.get()?)?;
                Ok(LenderIdentity::Individual(lender_, person))
            }
            Lender {
                company_id: Some(company_id),
                ..
            } => {
                let company = company::table.find(company_id).first(&self.0.get()?)?;
                Ok(LenderIdentity::Company(lender_, company))
            }
            _ => Err(Error::new(NotFound)),
        }
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

impl database::PropertyStore for PropertyStore<'_> {
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

    fn by_id(&mut self, id: PropertyId) -> Result<Property> {
        property::table
            .find(id)
            .first(&self.0.get()?)
            .map_err(|err| err.into())
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

    fn delete(&mut self, data: PropertyId) -> Result<Deleted> {
        Ok(delete(property::table)
            .filter(property::id.eq(data))
            .execute(&self.0.get()?)?)
    }

    fn update(&mut self, data: PropertyData) -> Result<Property> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }
}

impl database::LeaseStore for LeaseStore<'_> {
    fn by_id(&mut self, id: LeaseId) -> Result<Lease> {
        lease::table
            .find(id)
            .first(&self.0.get()?)
            .map_err(|err| err.into())
    }

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

    fn delete(&mut self, data: LeaseId) -> Result<Deleted> {
        Ok(delete(lease::table)
            .filter(lease::id.eq(data))
            .execute(&self.0.get()?)?)
    }

    fn update(&mut self, data: LeaseData) -> Result<Lease> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }
}

impl database::LeaseTenantStore for LeaseTenantStore<'_> {
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

impl database::RentStore for RentStore<'_> {
    fn by_id(&mut self, id: &RentId) -> Result<Rent> {
        rent::table
            .find(id)
            .first(&self.0.get()?)
            .map_err(|err| err.into())
    }

    fn by_receipt_id(&mut self, receipt_id: ReceiptId) -> Result<Rent> {
        rent::table
            .filter(rent::receipt_id.eq(&receipt_id))
            .first(&self.0.get()?)
            .map_err(|err| err.into())
    }

    fn by_lease_id(&mut self, lease_id: LeaseId) -> Result<Vec<Rent>> {
        rent::table
            .filter(rent::lease_id.eq(&lease_id))
            .load(&self.0.get()?)
            .map_err(|err| err.into())
    }

    fn create(&mut self, data: &Rent) -> Result<Rent> {
        Ok(insert_into(rent::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn create_many(&mut self, data: Vec<Rent>) -> Result<Vec<Rent>> {
        data.iter().map(|item| self.create(item)).collect()
    }

    fn update(&mut self, data: &RentData) -> Result<Rent> {
        Ok(update(data).set(data).get_result(&self.0.get()?)?)
    }

    fn update_many(&mut self, data: Vec<RentData>) -> Result<Vec<Rent>> {
        data.iter().map(|item| self.update(item)).collect()
    }
}

impl database::FileStore for FileStore<'_> {
    fn by_external_id(&mut self, external_id: ExternalId) -> Result<File> {
        file::table
            .filter(file::external_id.eq(external_id))
            .first(&self.0.get()?)
            .map_err(|err| err.into())
    }

    fn by_id(&mut self, id: FileId) -> Result<File> {
        file::table
            .find(id)
            .first(&self.0.get()?)
            .map_err(|err| err.into())
    }

    fn create(&mut self, data: &File) -> Result<File> {
        Ok(insert_into(file::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: &FileData) -> Result<File> {
        Ok(update(data).set(data).get_result(&self.0.get()?)?)
    }
}
