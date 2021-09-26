use crate::database;
use crate::database::Db;
use crate::error::Context;
use crate::error::Error;
use crate::providers::Provider;
use diesel::delete;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error::NotFound;
use diesel::update;
use diesel::PgConnection;
use piteo_data::schema::accounts;
use piteo_data::schema::advertisements;
use piteo_data::schema::candidacies;
use piteo_data::schema::companies;
use piteo_data::schema::events;
use piteo_data::schema::files;
use piteo_data::schema::leases;
use piteo_data::schema::lenders;
use piteo_data::schema::payments;
use piteo_data::schema::persons;
use piteo_data::schema::plans;
use piteo_data::schema::professional_warrants;
use piteo_data::schema::properties;
use piteo_data::schema::rents;
use piteo_data::schema::tenants;
use piteo_data::schema::warrants;
use piteo_data::Account;
use piteo_data::AccountData;
use piteo_data::AccountId;
use piteo_data::Advertisement;
use piteo_data::AdvertisementId;
use piteo_data::AuthId;
use piteo_data::Candidacy;
use piteo_data::Company;
use piteo_data::CompanyId;
use piteo_data::Event;
use piteo_data::EventId;
use piteo_data::EventWithEventable;
use piteo_data::Eventable;
use piteo_data::EventableType;
use piteo_data::File;
use piteo_data::FileData;
use piteo_data::FileId;
use piteo_data::Lease;
use piteo_data::LeaseData;
use piteo_data::LeaseId;
use piteo_data::LegalIdentity;
use piteo_data::Lender;
use piteo_data::LenderData;
use piteo_data::LenderId;
use piteo_data::LenderWithIdentity;
use piteo_data::Payment;
use piteo_data::Person;
use piteo_data::PersonData;
use piteo_data::PersonId;
use piteo_data::Plan;
use piteo_data::PlanId;
use piteo_data::ProfessionalWarrant;
use piteo_data::Property;
use piteo_data::PropertyData;
use piteo_data::PropertyId;
use piteo_data::ReceiptId;
use piteo_data::Rent;
use piteo_data::RentData;
use piteo_data::RentId;
use piteo_data::Tenant;
use piteo_data::TenantData;
use piteo_data::TenantId;
use piteo_data::Warrant;
use piteo_data::WarrantIdentity;
use piteo_data::WarrantType;
use piteo_data::WarrantWithIdentity;
use std::env;

type Result<T, E = Error> = std::result::Result<T, E>;

type Deleted = usize;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

struct AccountStore<'a>(&'a DbPool);

struct EventStore<'a>(&'a DbPool);

struct PersonStore<'a>(&'a DbPool);

struct CompanyStore<'a>(&'a DbPool);

struct TenantStore<'a>(&'a DbPool);

struct WarrantStore<'a>(&'a DbPool);

struct LenderStore<'a>(&'a DbPool);

struct AdvertisementStore<'a>(&'a DbPool);

struct CandidacyStore<'a>(&'a DbPool);

struct PropertyStore<'a>(&'a DbPool);

struct LeaseStore<'a>(&'a DbPool);

struct LeaseTenantStore<'a>(&'a DbPool);

struct RentStore<'a>(&'a DbPool);

struct FileStore<'a>(&'a DbPool);

struct PaymentStore<'a>(&'a DbPool);

struct PlanStore<'a>(&'a DbPool);

pub struct Pg(DbPool);

impl Pg {
    pub fn new(db_pool: DbPool) -> Self {
        Self(db_pool)
    }

    pub fn inner(&self) -> DbPool {
        self.0.clone()
    }
}

impl Provider for Pg {
    fn init() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
        Self::new(build_connection_pool(&database_url).unwrap())
    }
}

impl Db for Pg {
    fn accounts(&self) -> Box<dyn database::AccountStore + '_> {
        Box::new(AccountStore(&self.0))
    }

    fn persons(&self) -> Box<dyn database::PersonStore + '_> {
        Box::new(PersonStore(&self.0))
    }

    fn companies(&self) -> Box<dyn database::CompanyStore + '_> {
        Box::new(CompanyStore(&self.0))
    }

    fn lenders(&self) -> Box<dyn database::LenderStore + '_> {
        Box::new(LenderStore(&self.0))
    }

    fn tenants(&self) -> Box<dyn database::TenantStore + '_> {
        Box::new(TenantStore(&self.0))
    }

    fn warrants(&self) -> Box<dyn database::WarrantStore + '_> {
        Box::new(WarrantStore(&self.0))
    }

    fn advertisements(&self) -> Box<dyn database::AdvertisementStore + '_> {
        Box::new(AdvertisementStore(&self.0))
    }

    fn candidacies(&self) -> Box<dyn database::CandidacyStore + '_> {
        Box::new(CandidacyStore(&self.0))
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

    fn payments(&self) -> Box<dyn database::PaymentStore + '_> {
        Box::new(PaymentStore(&self.0))
    }

    fn plans(&self) -> Box<dyn database::PlanStore + '_> {
        Box::new(PlanStore(&self.0))
    }

    fn events(&self) -> Box<dyn database::EventStore + '_> {
        Box::new(EventStore(&self.0))
    }
}

impl database::AccountStore for AccountStore<'_> {
    fn by_id(&mut self, id: &AccountId) -> Result<Account> {
        Ok(accounts::table.find(id).first(&self.0.get()?)?)
    }

    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Account> {
        Ok(accounts::table
            .select(accounts::all_columns)
            .left_join(persons::table.on(persons::account_id.eq(accounts::id)))
            .filter(persons::auth_id.eq(&auth_id.inner()))
            .first(&self.0.get()?)?)
    }

    fn by_advertisement_id(&mut self, advertisement_id: &AdvertisementId) -> Result<Account> {
        Ok(accounts::table
            .select(accounts::all_columns)
            .left_join(properties::table.on(properties::account_id.eq(accounts::id)))
            .left_join(advertisements::table.on(advertisements::property_id.eq(properties::id)))
            .filter(advertisements::id.eq(&advertisement_id))
            .first(&self.0.get()?)?)
    }

    fn create(&mut self, data: Account) -> Result<Account> {
        Ok(insert_into(accounts::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: AccountData) -> Result<Account> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }
}

impl database::PersonStore for PersonStore<'_> {
    fn by_id(&mut self, id: &PersonId) -> Result<Person> {
        Ok(persons::table.find(id).first(&self.0.get()?)?)
    }

    fn by_account_id(&mut self, account_id: &AccountId) -> Result<Vec<Person>> {
        Ok(persons::table
            .filter(persons::account_id.eq(account_id))
            .load(&self.0.get()?)?)
    }

    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Person> {
        Ok(persons::table
            .filter(persons::auth_id.eq(auth_id.inner()))
            .first(&self.0.get()?)?)
    }

    fn create(&mut self, data: Person) -> Result<Person> {
        Ok(insert_into(persons::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: PersonData) -> Result<Person> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }
}

impl database::TenantStore for TenantStore<'_> {
    fn by_id(&mut self, id: &TenantId) -> Result<Tenant> {
        Ok(tenants::table.find(id).first(&self.0.get()?)?)
    }

    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Tenant>> {
        Ok(tenants::table
            .select(tenants::all_columns)
            .left_join(persons::table.on(persons::account_id.eq(tenants::account_id)))
            .filter(persons::auth_id.eq(auth_id.inner()))
            .load(&self.0.get()?)?)
    }

    fn by_lease_id(&mut self, lease_id: &LeaseId) -> Result<Vec<Tenant>> {
        Ok(tenants::table
            .filter(tenants::lease_id.eq(lease_id))
            .load(&self.0.get()?)?)
    }

    fn create(&mut self, data: Tenant) -> Result<Tenant> {
        Ok(insert_into(tenants::table)
            .values(data)
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

impl database::WarrantStore for WarrantStore<'_> {
    fn by_tenant_id(&mut self, tenant_id: &TenantId) -> Result<Vec<WarrantWithIdentity>> {
        warrants::table
            .filter(warrants::tenant_id.eq(tenant_id))
            .load::<Warrant>(&self.0.get()?)?
            .into_iter()
            .map(|warrant| {
                let identity = match (
                    warrant.type_,
                    warrant.individual_id,
                    warrant.professional_id,
                ) {
                    (WarrantType::Person, Some(individual_id), _) => {
                        let person = persons::table.find(individual_id).first(&self.0.get()?)?;
                        WarrantIdentity::Individual(person)
                    }
                    (WarrantType::Visale, _, Some(professional_id)) => {
                        let visale = professional_warrants::table
                            .find(professional_id)
                            .first(&self.0.get()?)?;
                        WarrantIdentity::Professional(visale)
                    }
                    (WarrantType::Company, _, Some(professional_id)) => {
                        let company = professional_warrants::table
                            .find(professional_id)
                            .first(&self.0.get()?)?;
                        WarrantIdentity::Professional(company)
                    }
                    _ => return Err(Error::new(NotFound)),
                };

                Ok((warrant, identity))
            })
            .collect()
    }

    fn create(&mut self, data: WarrantWithIdentity) -> Result<WarrantWithIdentity> {
        match (data.0.type_, data.1) {
            (WarrantType::Person, WarrantIdentity::Individual(person)) => {
                let person = insert_into(persons::table)
                    .values(person)
                    .get_result::<Person>(&self.0.get()?)?;

                let warrant = insert_into(warrants::table)
                    .values(Warrant {
                        individual_id: Some(person.id),
                        ..data.0
                    })
                    .get_result(&self.0.get()?)?;

                Ok((warrant, WarrantIdentity::Individual(person)))
            }
            (WarrantType::Visale, WarrantIdentity::Professional(professional)) => {
                let professional = insert_into(professional_warrants::table)
                    .values(professional)
                    .get_result::<ProfessionalWarrant>(&self.0.get()?)?;

                let warrant = insert_into(warrants::table)
                    .values(Warrant {
                        professional_id: Some(professional.id),
                        ..data.0
                    })
                    .get_result(&self.0.get()?)?;

                Ok((warrant, WarrantIdentity::Professional(professional)))
            }
            (WarrantType::Company, WarrantIdentity::Professional(professional)) => {
                let professional = insert_into(professional_warrants::table)
                    .values(professional)
                    .get_result::<ProfessionalWarrant>(&self.0.get()?)?;

                let warrant = insert_into(warrants::table)
                    .values(Warrant {
                        professional_id: Some(professional.id),
                        ..data.0
                    })
                    .get_result(&self.0.get()?)?;

                Ok((warrant, WarrantIdentity::Professional(professional)))
            }
            _ => Err(Error::new(NotFound)),
        }
    }
}

impl database::LenderStore for LenderStore<'_> {
    fn by_id(&mut self, id: &LenderId) -> Result<LenderWithIdentity> {
        let lender: Lender = lenders::table.find(id).first(&self.0.get()?)?;

        let identity = match (lender.individual_id, lender.company_id) {
            (Some(individual_id), _) => {
                let person = persons::table.find(individual_id).first(&self.0.get()?)?;
                LegalIdentity::Individual(person)
            }
            (_, Some(company_id)) => {
                let company = companies::table.find(company_id).first(&self.0.get()?)?;
                LegalIdentity::Company(company)
            }
            _ => return Err(Error::new(NotFound)),
        };

        Ok((lender, identity))
    }

    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<LenderWithIdentity>> {
        lenders::table
            .select(lenders::all_columns)
            .left_join(persons::table.on(persons::account_id.eq(lenders::account_id)))
            .filter(persons::auth_id.eq(auth_id.inner()))
            .load(&self.0.get()?)?
            .iter()
            .map(|lender: &Lender| self.by_id(&lender.id))
            .collect::<Result<Vec<_>>>()
    }

    fn by_individual_id(&mut self, individual_id: &PersonId) -> Result<LenderWithIdentity> {
        let lender: Lender = lenders::table
            .filter(lenders::individual_id.eq(individual_id))
            .first(&self.0.get()?)?;
        let person = persons::table.find(individual_id).first(&self.0.get()?)?;
        Ok((lender, LegalIdentity::Individual(person)))
    }

    fn create(&mut self, data: Lender) -> Result<Lender> {
        Ok(insert_into(lenders::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: LenderData) -> Result<Lender> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }
}

impl database::AdvertisementStore for AdvertisementStore<'_> {
    fn by_id(&mut self, id: &AdvertisementId) -> Result<Advertisement> {
        Ok(advertisements::table.find(id).first(&self.0.get()?)?)
    }

    fn by_property_id(&mut self, property_id: &PropertyId) -> Result<Vec<Advertisement>> {
        Ok(advertisements::table
            .filter(advertisements::property_id.eq(property_id))
            .load(&self.0.get()?)?)
    }

    fn create(&mut self, data: Advertisement) -> Result<Advertisement> {
        Ok(insert_into(advertisements::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }
}

impl database::CandidacyStore for CandidacyStore<'_> {
    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Candidacy>> {
        Ok(candidacies::table
            .select(candidacies::all_columns)
            .left_join(tenants::table.on(tenants::id.eq(candidacies::tenant_id)))
            .left_join(persons::table.on(persons::account_id.eq(tenants::account_id)))
            .filter(persons::auth_id.eq(auth_id.inner()))
            .load(&self.0.get()?)?)
    }

    fn by_property_id(&mut self, property_id: &PropertyId) -> Result<Vec<Candidacy>> {
        Ok(candidacies::table
            .select(candidacies::all_columns)
            .left_join(
                advertisements::table.on(advertisements::id.eq(candidacies::advertisement_id)),
            )
            .left_join(properties::table.on(properties::id.eq(advertisements::property_id)))
            .filter(properties::id.eq(property_id))
            .load(&self.0.get()?)?)
    }

    fn create(&mut self, data: Candidacy) -> Result<Candidacy> {
        Ok(insert_into(candidacies::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }
}

impl database::PropertyStore for PropertyStore<'_> {
    fn by_id(&mut self, id: &PropertyId) -> Result<Property> {
        Ok(properties::table.find(id).first(&self.0.get()?)?)
    }

    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Property>> {
        Ok(properties::table
            .select(properties::all_columns)
            .left_join(persons::table.on(persons::account_id.eq(properties::account_id)))
            .filter(persons::auth_id.eq(auth_id.inner()))
            .load(&self.0.get()?)?)
    }

    fn create(&mut self, data: Property) -> Result<Property> {
        Ok(insert_into(properties::table)
            .values(data)
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
    fn by_id(&mut self, id: &LeaseId) -> Result<Lease> {
        Ok(leases::table.find(id).first(&self.0.get()?)?)
    }

    fn by_property_id(&mut self, property_id: &PropertyId) -> Result<Vec<Lease>> {
        Ok(leases::table
            .filter(leases::property_id.eq(property_id))
            .load(&self.0.get()?)?)
    }

    fn by_receipt_id(&mut self, receipt_id: &ReceiptId) -> Result<Lease> {
        Ok(leases::table
            .select(leases::all_columns)
            .left_join(rents::table.on(rents::lease_id.eq(leases::id)))
            .filter(rents::receipt_id.eq(receipt_id))
            .first(&self.0.get()?)?)
    }

    fn by_rent_id(&mut self, rent_id: &RentId) -> Result<Lease> {
        Ok(leases::table
            .select(leases::all_columns)
            .left_join(rents::table.on(rents::lease_id.eq(leases::id)))
            .filter(rents::id.eq(rent_id))
            .first(&self.0.get()?)?)
    }

    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Lease>> {
        Ok(leases::table
            .select(leases::all_columns)
            .left_join(persons::table.on(persons::account_id.eq(leases::account_id)))
            .filter(persons::auth_id.eq(auth_id.inner()))
            .load(&self.0.get()?)?)
    }

    fn create(&mut self, data: Lease) -> Result<Lease> {
        Ok(insert_into(leases::table)
            .values(data)
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
        Ok(rents::table.find(id).first(&self.0.get()?)?)
    }

    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Rent>> {
        Ok(rents::table
            .select(rents::all_columns)
            .left_join(leases::table.on(leases::id.eq(rents::lease_id)))
            .left_join(persons::table.on(persons::account_id.eq(leases::account_id)))
            .filter(persons::auth_id.eq(auth_id.inner()))
            .load(&self.0.get()?)?)
    }

    fn by_receipt_id(&mut self, receipt_id: &ReceiptId) -> Result<Rent> {
        Ok(rents::table
            .filter(rents::receipt_id.eq(&receipt_id))
            .first(&self.0.get()?)?)
    }

    fn by_lease_id(&mut self, lease_id: &LeaseId) -> Result<Vec<Rent>> {
        Ok(rents::table
            .filter(rents::lease_id.eq(&lease_id))
            .load(&self.0.get()?)?)
    }

    fn create(&mut self, data: Rent) -> Result<Rent> {
        Ok(insert_into(rents::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn create_many(&mut self, data: Vec<Rent>) -> Result<Vec<Rent>> {
        data.into_iter()
            .map(|item| self.create(item))
            .collect::<Result<Vec<_>>>()
    }

    fn update(&mut self, data: RentData) -> Result<Rent> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }
}

impl database::FileStore for FileStore<'_> {
    fn by_external_id(&mut self, external_id: &str) -> Result<File> {
        Ok(files::table
            .filter(files::external_id.eq(external_id))
            .first(&self.0.get()?)?)
    }

    fn by_id(&mut self, id: &FileId) -> Result<File> {
        Ok(files::table.find(id).first(&self.0.get()?)?)
    }

    fn create(&mut self, data: File) -> Result<File> {
        Ok(insert_into(files::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: FileData) -> Result<File> {
        Ok(update(&data).set(&data).get_result(&self.0.get()?)?)
    }
}

impl database::PaymentStore for PaymentStore<'_> {
    fn create(&mut self, data: Payment) -> Result<Payment> {
        Ok(insert_into(payments::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }
}

impl database::PlanStore for PlanStore<'_> {
    fn by_id(&mut self, id: &PlanId) -> Result<Plan> {
        Ok(plans::table.find(id).first(&self.0.get()?)?)
    }
}

impl database::CompanyStore for CompanyStore<'_> {
    fn by_id(&mut self, id: &CompanyId) -> Result<Company> {
        Ok(companies::table.find(id).first(&self.0.get()?)?)
    }
}

impl database::EventStore for EventStore<'_> {
    fn by_id(&mut self, id: &EventId) -> Result<EventWithEventable> {
        let event: Event = events::table.find(id).first(&self.0.get()?)?;
        let detailed_event = match event.eventable_type {
            EventableType::Rent => {
                let rent = rents::table
                    .find(event.eventable_id)
                    .first(&self.0.get()?)?;
                (event, Eventable::Rent(rent))
            }
            EventableType::Payment => {
                let payment = payments::table
                    .find(event.eventable_id)
                    .first(&self.0.get()?)?;
                (event, Eventable::Payment(payment))
            }
        };
        Ok(detailed_event)
    }

    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<EventWithEventable>> {
        events::table
            .select(events::all_columns)
            .left_join(persons::table.on(persons::account_id.eq(events::account_id)))
            .filter(persons::auth_id.eq(auth_id.inner()))
            .load(&self.0.get()?)?
            .iter()
            .map(|event: &Event| self.by_id(&event.id))
            .collect::<Result<Vec<_>>>()
    }

    fn create(&mut self, data: Event) -> Result<Event> {
        Ok(insert_into(events::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }
}

// # Utils

fn build_connection_pool(database_url: &str) -> Result<DbPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .context(format!("Error connecting to {}", database_url))
}
