use crate::Provider;
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
use piteo_core::schema::accounts;
use piteo_core::schema::companies;
use piteo_core::schema::files;
use piteo_core::schema::leases;
use piteo_core::schema::lenders;
use piteo_core::schema::persons;
use piteo_core::schema::properties;
use piteo_core::schema::rents;
use piteo_core::schema::tenants;
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
use piteo_core::Lender;
use piteo_core::LenderData;
use piteo_core::LenderId;
use piteo_core::LenderIdentity;
use piteo_core::Person;
use piteo_core::PersonData;
use piteo_core::PersonId;
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
pub fn db(db_pool: DbPool) -> Pg {
    Pg::new(db_pool)
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
    pub fn new(db_pool: DbPool) -> Self {
        Self(db_pool)
    }
}

impl Provider for Pg {
    fn init() -> Self {
        Self::new(db_pool_from_env().unwrap())
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

    fn rents(&self) -> Box<dyn database::RentStore + '_> {
        Box::new(RentStore(&self.0))
    }

    fn files(&self) -> Box<dyn database::FileStore + '_> {
        Box::new(FileStore(&self.0))
    }
}

impl database::AccountStore for AccountStore<'_> {
    fn by_auth_id(&mut self, auth_id: AuthId) -> Result<Account> {
        accounts::table
            .select(accounts::all_columns)
            .left_join(persons::table.on(persons::account_id.eq(accounts::id)))
            .filter(persons::auth_id.eq(&auth_id.inner()))
            .first(&self.0.get()?)
            .map_err(|err| err.into())
    }

    fn create(&mut self, data: Account) -> Result<Account> {
        Ok(insert_into(accounts::table)
            .values((
                accounts::plan_id.eq(data.plan_id),
                accounts::status.eq(data.status),
                accounts::stripe_customer_id.eq(data.stripe_customer_id),
                accounts::stripe_subscription_id.eq(data.stripe_subscription_id),
                accounts::trial_end.eq(data.trial_end),
            ))
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: AccountData) -> Result<Account> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }
}

impl database::UserStore for UserStore<'_> {
    fn create(&mut self, data: Person) -> Result<Person> {
        Ok(insert_into(persons::table)
            .values((
                persons::auth_id.eq(data.auth_id),
                persons::email.eq(data.email),
                persons::first_name.eq(data.first_name),
                persons::last_name.eq(data.last_name),
                persons::address.eq(data.address),
                persons::photo_url.eq(data.photo_url),
                persons::role.eq(data.role),
                persons::phone_number.eq(data.phone_number),
                persons::account_id.eq(data.account_id),
            ))
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: PersonData) -> Result<Person> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }
}

impl database::TenantStore for TenantStore<'_> {
    fn all(&mut self, auth_id: AuthId, id: Option<TenantId>) -> Result<Vec<Tenant>> {
        let query = tenants::table
            .select(tenants::all_columns)
            .left_join(persons::table.on(persons::account_id.eq(tenants::account_id)))
            .filter(persons::auth_id.eq(auth_id.inner()));

        match id {
            Some(id) => query
                .filter(tenants::id.eq(id))
                .load(&self.0.get()?)
                .map_err(|err| err.into()),
            None => query.load(&self.0.get()?).map_err(|err| err.into()),
        }
    }

    fn by_lease_id(&mut self, lease_id: LeaseId) -> Result<Vec<Tenant>> {
        tenants::table
            .filter(tenants::lease_id.eq(lease_id))
            .load(&self.0.get()?)
            .map_err(|err| err.into())
    }

    fn create(&mut self, data: Tenant) -> Result<Tenant> {
        Ok(insert_into(tenants::table)
            .values((
                tenants::account_id.eq(data.account_id),
                tenants::apl.eq(data.apl),
                tenants::birthdate.eq(data.birthdate),
                tenants::birthplace.eq(data.birthplace),
                tenants::email.eq(data.email),
                tenants::first_name.eq(data.first_name),
                tenants::last_name.eq(data.last_name),
                tenants::note.eq(data.note),
                tenants::phone_number.eq(data.phone_number),
                tenants::visale_id.eq(data.visale_id),
            ))
            .get_result(&self.0.get()?)?)
    }

    fn delete(&mut self, data: TenantId) -> Result<Deleted> {
        Ok(delete(tenants::table)
            .filter(tenants::id.eq(data))
            .execute(&self.0.get()?)?)
    }

    fn update(&mut self, data: TenantData) -> Result<Tenant> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }
}

impl database::LenderStore for LenderStore<'_> {
    fn by_id(&mut self, id: LenderId) -> Result<LenderIdentity> {
        let lender: Lender = lenders::table.find(id).first(&self.0.get()?)?;

        match lender {
            Lender {
                individual_id: Some(individual_id),
                ..
            } => {
                let person = persons::table.find(individual_id).first(&self.0.get()?)?;
                Ok(LenderIdentity::Individual(lender, person))
            }
            Lender {
                company_id: Some(company_id),
                ..
            } => {
                let company = companies::table.find(company_id).first(&self.0.get()?)?;
                Ok(LenderIdentity::Company(lender, company))
            }
            _ => Err(Error::new(NotFound)),
        }
    }

    fn by_individual_id(&mut self, individual_id: PersonId) -> Result<LenderIdentity> {
        let lender: Lender = lenders::table
            .filter(lenders::individual_id.eq(individual_id))
            .first(&self.0.get()?)?;
        let person = persons::table.find(individual_id).first(&self.0.get()?)?;
        Ok(LenderIdentity::Individual(lender, person))
    }

    fn create(&mut self, data: Lender) -> Result<Lender> {
        Ok(insert_into(lenders::table)
            .values((
                lenders::account_id.eq(data.account_id),
                lenders::individual_id.eq(data.individual_id),
                lenders::company_id.eq(data.company_id),
            ))
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: LenderData) -> Result<Lender> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }
}

impl database::PropertyStore for PropertyStore<'_> {
    fn all(&mut self, auth_id: AuthId, id: Option<PropertyId>) -> Result<Vec<Property>> {
        let query = properties::table
            .select(properties::all_columns)
            .left_join(persons::table.on(persons::account_id.eq(properties::account_id)))
            .filter(persons::auth_id.eq(auth_id.inner()));

        if let Some(id) = id {
            return query
                .filter(properties::id.eq(id))
                .load(&self.0.get()?)
                .map_err(|err| err.into());
        }

        query.load(&self.0.get()?).map_err(|err| err.into())
    }

    fn by_id(&mut self, id: PropertyId) -> Result<Property> {
        properties::table
            .find(id)
            .first(&self.0.get()?)
            .map_err(|err| err.into())
    }

    fn create(&mut self, data: Property) -> Result<Property> {
        Ok(insert_into(properties::table)
            .values((
                properties::account_id.eq(data.account_id),
                properties::address.eq(data.address),
                properties::build_period.eq(data.build_period),
                properties::building_legal_status.eq(data.building_legal_status),
                properties::common_spaces.eq(data.common_spaces),
                properties::energy_class.eq(data.energy_class),
                properties::equipments.eq(data.equipments),
                properties::gas_emission.eq(data.gas_emission),
                properties::heating_method.eq(data.heating_method),
                properties::housing_type.eq(data.housing_type),
                properties::name.eq(data.name),
                properties::note.eq(data.note),
                properties::ntic_equipments.eq(data.ntic_equipments),
                properties::other_spaces.eq(data.other_spaces),
                properties::tax.eq(data.tax),
                properties::room_count.eq(data.room_count),
                properties::status.eq(data.status),
                properties::surface.eq(data.surface),
                properties::tenant_private_spaces.eq(data.tenant_private_spaces),
                properties::usage_type.eq(data.usage_type),
                properties::water_heating_method.eq(data.water_heating_method),
                properties::lender_id.eq(data.lender_id),
            ))
            .get_result(&self.0.get()?)?)
    }

    fn delete(&mut self, data: PropertyId) -> Result<Deleted> {
        Ok(delete(properties::table)
            .filter(properties::id.eq(data))
            .execute(&self.0.get()?)?)
    }

    fn update(&mut self, data: PropertyData) -> Result<Property> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }
}

impl database::LeaseStore for LeaseStore<'_> {
    fn by_id(&mut self, id: LeaseId) -> Result<Lease> {
        leases::table
            .find(id)
            .first(&self.0.get()?)
            .map_err(|err| err.into())
    }

    fn create(&mut self, data: Lease) -> Result<Lease> {
        Ok(insert_into(leases::table)
            .values((
                leases::account_id.eq(data.account_id),
                leases::deposit_amount.eq(data.deposit_amount),
                leases::effect_date.eq(data.effect_date),
                leases::signature_date.eq(data.signature_date),
                leases::rent_amount.eq(data.rent_amount),
                leases::rent_charges_amount.eq(data.rent_charges_amount),
                leases::type_.eq(data.type_),
                leases::lease_id.eq(data.lease_id),
                leases::property_id.eq(data.property_id),
                leases::details.eq(data.details),
                leases::expired_at.eq(data.expired_at),
                leases::renew_date.eq(data.renew_date),
                leases::duration.eq(data.duration),
            ))
            .get_result(&self.0.get()?)?)
    }

    fn delete(&mut self, data: LeaseId) -> Result<Deleted> {
        Ok(delete(leases::table)
            .filter(leases::id.eq(data))
            .execute(&self.0.get()?)?)
    }

    fn update(&mut self, data: LeaseData) -> Result<Lease> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }
}

impl database::RentStore for RentStore<'_> {
    fn by_id(&mut self, id: &RentId) -> Result<Rent> {
        rents::table
            .find(id)
            .first(&self.0.get()?)
            .map_err(|err| err.into())
    }

    fn by_receipt_id(&mut self, receipt_id: ReceiptId) -> Result<Rent> {
        rents::table
            .filter(rents::receipt_id.eq(&receipt_id))
            .first(&self.0.get()?)
            .map_err(|err| err.into())
    }

    fn by_lease_id(&mut self, lease_id: LeaseId) -> Result<Vec<Rent>> {
        rents::table
            .filter(rents::lease_id.eq(&lease_id))
            .load(&self.0.get()?)
            .map_err(|err| err.into())
    }

    fn create(&mut self, data: &Rent) -> Result<Rent> {
        Ok(insert_into(rents::table)
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
        files::table
            .filter(files::external_id.eq(external_id))
            .first(&self.0.get()?)
            .map_err(|err| err.into())
    }

    fn by_id(&mut self, id: FileId) -> Result<File> {
        files::table
            .find(id)
            .first(&self.0.get()?)
            .map_err(|err| err.into())
    }

    fn create(&mut self, data: &File) -> Result<File> {
        Ok(insert_into(files::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: &FileData) -> Result<File> {
        Ok(update(data).set(data).get_result(&self.0.get()?)?)
    }
}
