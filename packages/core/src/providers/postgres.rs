use crate::database;
use crate::database::Db;
use crate::database::Executed;
use crate::database::Result;
use crate::error::Error;
use diesel::delete;
use diesel::dsl::now;
use diesel::insert_into;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error::NotFound;
use diesel::update;
use diesel::PgConnection;
use std::env;
use trankeel_data::balances;
use trankeel_data::reports;
use trankeel_data::schema::accounts;
use trankeel_data::schema::advertisements;
use trankeel_data::schema::candidacies;
use trankeel_data::schema::companies;
use trankeel_data::schema::discussions;
use trankeel_data::schema::eventables;
use trankeel_data::schema::events;
use trankeel_data::schema::files;
use trankeel_data::schema::invites;
use trankeel_data::schema::leases;
use trankeel_data::schema::lenders;
use trankeel_data::schema::messages;
use trankeel_data::schema::payments;
use trankeel_data::schema::persons;
use trankeel_data::schema::plans;
use trankeel_data::schema::professional_warrants;
use trankeel_data::schema::properties;
use trankeel_data::schema::rents;
use trankeel_data::schema::steps;
use trankeel_data::schema::tenants;
use trankeel_data::schema::warrants;
use trankeel_data::schema::workflowables;
use trankeel_data::schema::workflows;
use trankeel_data::Account;
use trankeel_data::AccountId;
use trankeel_data::Advertisement;
use trankeel_data::AdvertisementId;
use trankeel_data::AuthId;
use trankeel_data::Balance;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyId;
use trankeel_data::Company;
use trankeel_data::CompanyId;
use trankeel_data::Discussion;
use trankeel_data::DiscussionId;
use trankeel_data::DiscussionItem;
use trankeel_data::DiscussionItemRow;
use trankeel_data::Event;
use trankeel_data::EventId;
use trankeel_data::EventWithEventable;
use trankeel_data::Eventable;
use trankeel_data::EventableRow;
use trankeel_data::ExternalId;
use trankeel_data::File;
use trankeel_data::FileId;
use trankeel_data::Invite;
use trankeel_data::InviteToken;
use trankeel_data::Lease;
use trankeel_data::LeaseFileId;
use trankeel_data::LeaseId;
use trankeel_data::LegalIdentity;
use trankeel_data::Lender;
use trankeel_data::LenderId;
use trankeel_data::LenderWithIdentity;
use trankeel_data::Message;
use trankeel_data::NoticeId;
use trankeel_data::Payment;
use trankeel_data::PaymentId;
use trankeel_data::Person;
use trankeel_data::PersonId;
use trankeel_data::Plan;
use trankeel_data::PlanId;
use trankeel_data::ProfessionalWarrant;
use trankeel_data::Property;
use trankeel_data::PropertyId;
use trankeel_data::ReceiptId;
use trankeel_data::Rent;
use trankeel_data::RentId;
use trankeel_data::Step;
use trankeel_data::StepId;
use trankeel_data::Summary;
use trankeel_data::Tenant;
use trankeel_data::TenantId;
use trankeel_data::TenantWithBalance;
use trankeel_data::Warrant;
use trankeel_data::WarrantId;
use trankeel_data::WarrantIdentity;
use trankeel_data::WarrantType;
use trankeel_data::WarrantWithIdentity;
use trankeel_data::Workflow;
use trankeel_data::WorkflowId;
use trankeel_data::WorkflowWithSteps;
use trankeel_data::Workflowable;
use trankeel_data::WorkflowableId;

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct Pg(PgPool);

impl Pg {
    pub fn init() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
        let manager = ConnectionManager::<PgConnection>::new(&database_url);
        Self(
            Pool::builder()
                .build(manager)
                .expect("Error connecting to database"),
        )
    }

    pub fn transaction<T, F>(&self, f: F) -> Result<T>
    where
        F: FnOnce() -> Result<T>,
    {
        self.0.get()?.transaction(f)
    }
}

impl Db for Pg {
    fn accounts(&self) -> Box<dyn database::AccountStore + '_> {
        Box::new(AccountStore(&self.0))
    }

    fn balances(&self) -> Box<dyn database::BalanceStore + '_> {
        Box::new(BalanceStore(&self.0))
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

    fn eventables(&self) -> Box<dyn database::EventableStore + '_> {
        Box::new(EventableStore(&self.0))
    }

    fn reports(&self) -> Box<dyn database::ReportStore + '_> {
        Box::new(ReportStore(&self.0))
    }

    fn discussions(&self) -> Box<dyn database::DiscussionStore + '_> {
        Box::new(DiscussionStore(&self.0))
    }

    fn messages(&self) -> Box<dyn database::MessageStore + '_> {
        Box::new(MessageStore(&self.0))
    }

    fn invites(&self) -> Box<dyn database::InviteStore + '_> {
        Box::new(InviteStore(&self.0))
    }

    fn workflowables(&self) -> Box<dyn database::WorkflowableStore + '_> {
        Box::new(WorkflowableStore(&self.0))
    }

    fn workflows(&self) -> Box<dyn database::WorkflowStore + '_> {
        Box::new(WorkflowStore(&self.0))
    }

    fn steps(&self) -> Box<dyn database::StepStore + '_> {
        Box::new(StepStore(&self.0))
    }
}

pub struct AccountStore<'a>(pub &'a PgPool);

pub struct BalanceStore<'a>(pub &'a PgPool);

pub struct EventStore<'a>(pub &'a PgPool);

pub struct EventableStore<'a>(pub &'a PgPool);

pub struct PersonStore<'a>(pub &'a PgPool);

pub struct CompanyStore<'a>(pub &'a PgPool);

pub struct TenantStore<'a>(pub &'a PgPool);

pub struct WarrantStore<'a>(pub &'a PgPool);

pub struct LenderStore<'a>(pub &'a PgPool);

pub struct AdvertisementStore<'a>(pub &'a PgPool);

pub struct CandidacyStore<'a>(pub &'a PgPool);

pub struct PropertyStore<'a>(pub &'a PgPool);

pub struct LeaseStore<'a>(pub &'a PgPool);

pub struct LeaseTenantStore<'a>(pub &'a PgPool);

pub struct RentStore<'a>(pub &'a PgPool);

pub struct FileStore<'a>(pub &'a PgPool);

pub struct PaymentStore<'a>(pub &'a PgPool);

pub struct PlanStore<'a>(pub &'a PgPool);

pub struct ReportStore<'a>(pub &'a PgPool);

pub struct DiscussionStore<'a>(pub &'a PgPool);

pub struct MessageStore<'a>(pub &'a PgPool);

pub struct InviteStore<'a>(pub &'a PgPool);

pub struct WorkflowableStore<'a>(pub &'a PgPool);

pub struct WorkflowStore<'a>(pub &'a PgPool);

pub struct StepStore<'a>(pub &'a PgPool);

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
            .filter(advertisements::id.eq(advertisement_id))
            .first(&self.0.get()?)?)
    }

    fn by_candidacy_id(&mut self, candidacy_id: &CandidacyId) -> Result<Account> {
        Ok(accounts::table
            .select(accounts::all_columns)
            .left_join(persons::table.on(persons::account_id.eq(accounts::id)))
            .left_join(candidacies::table.on(candidacies::person_id.eq(persons::id)))
            .filter(candidacies::id.eq(candidacy_id))
            .first(&self.0.get()?)?)
    }

    fn by_notice_id(&mut self, notice_id: &NoticeId) -> Result<Account> {
        Ok(accounts::table
            .select(accounts::all_columns)
            .left_join(leases::table.on(leases::account_id.eq(accounts::id)))
            .left_join(rents::table.on(rents::lease_id.eq(leases::id)))
            .left_join(files::table.on(files::id.nullable().eq(rents::notice_id)))
            .filter(files::id.eq(notice_id))
            .first(&self.0.get()?)?)
    }

    fn by_receipt_id(&mut self, receipt_id: &ReceiptId) -> Result<Account> {
        Ok(accounts::table
            .select(accounts::all_columns)
            .left_join(leases::table.on(leases::account_id.eq(accounts::id)))
            .left_join(rents::table.on(rents::lease_id.eq(leases::id)))
            .left_join(files::table.on(files::id.nullable().eq(rents::receipt_id)))
            .filter(files::id.eq(receipt_id))
            .first(&self.0.get()?)?)
    }

    fn by_payment_id(&mut self, payment_id: &PaymentId) -> Result<Account> {
        Ok(accounts::table
            .select(accounts::all_columns)
            .left_join(leases::table.on(leases::account_id.eq(accounts::id)))
            .left_join(rents::table.on(rents::lease_id.eq(leases::id)))
            .left_join(payments::table.on(payments::rent_id.eq(rents::id)))
            .filter(payments::id.eq(payment_id))
            .first(&self.0.get()?)?)
    }

    fn by_person_id(&mut self, person_id: &PersonId) -> Result<Account> {
        Ok(accounts::table
            .select(accounts::all_columns)
            .left_join(persons::table.on(persons::account_id.eq(accounts::id)))
            .filter(persons::id.eq(person_id))
            .first(&self.0.get()?)?)
    }

    fn by_lease_id(&mut self, lease_id: &LeaseId) -> Result<Account> {
        Ok(accounts::table
            .select(accounts::all_columns)
            .left_join(leases::table.on(leases::account_id.eq(accounts::id)))
            .filter(leases::id.eq(lease_id))
            .first(&self.0.get()?)?)
    }

    fn by_step_id(&mut self, step_id: &StepId) -> Result<Account> {
        Ok(accounts::table
            .select(accounts::all_columns)
            .left_join(persons::table.on(persons::account_id.eq(accounts::id)))
            .left_join(candidacies::table.on(candidacies::person_id.eq(persons::id)))
            .left_join(workflows::table.on(workflows::workflowable_id.eq(candidacies::id))) // TODO: match workflowable
            .left_join(steps::table.on(steps::workflow_id.eq(workflows::id)))
            .filter(steps::id.eq(step_id))
            .first(&self.0.get()?)?)
    }

    fn create(&mut self, data: &Account) -> Result<Account> {
        Ok(insert_into(accounts::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: &Account) -> Result<Account> {
        Ok(update(data).set(data).get_result(&self.0.get()?)?)
    }
}

impl database::BalanceStore for BalanceStore<'_> {
    fn by_tenant_id(&mut self, tenant_id: &TenantId) -> Result<Balance> {
        Ok(balances::table
            .filter(balances::tenant_id.eq(tenant_id))
            .first(&self.0.get()?)?)
    }
}

impl database::PersonStore for PersonStore<'_> {
    fn by_id(&mut self, id: &PersonId) -> Result<Person> {
        Ok(persons::table.find(id).first(&self.0.get()?)?)
    }

    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Person> {
        Ok(persons::table
            .filter(persons::auth_id.eq(auth_id.inner()))
            .first(&self.0.get()?)?)
    }

    fn by_account_id(&mut self, account_id: &AccountId) -> Result<Vec<Person>> {
        Ok(persons::table
            .filter(persons::account_id.eq(account_id))
            .load(&self.0.get()?)?)
    }

    fn by_account_id_first(&mut self, account_id: &AccountId) -> Result<Person> {
        Ok(self
            .by_account_id(account_id)?
            .first()
            .cloned()
            .ok_or(NotFound)?)
    }

    fn by_candidacy_id(&mut self, candidacy_id: &CandidacyId) -> Result<Person> {
        Ok(persons::table
            .select(persons::all_columns)
            .left_join(candidacies::table.on(candidacies::person_id.eq(persons::id)))
            .filter(candidacies::id.eq(candidacy_id))
            .first(&self.0.get()?)?)
    }

    fn by_notice_id(&mut self, notice_id: &FileId) -> Result<Person> {
        Ok(persons::table
            .select(persons::all_columns)
            .left_join(tenants::table.on(tenants::person_id.eq(persons::id)))
            .left_join(leases::table.on(leases::id.nullable().eq(tenants::lease_id)))
            .left_join(rents::table.on(rents::lease_id.eq(leases::id)))
            .left_join(files::table.on(files::id.nullable().eq(rents::notice_id)))
            .filter(files::id.eq(notice_id))
            .first(&self.0.get()?)?)
    }

    fn by_receipt_id(&mut self, receipt_id: &ReceiptId) -> Result<Person> {
        Ok(persons::table
            .select(persons::all_columns)
            .left_join(tenants::table.on(tenants::person_id.eq(persons::id)))
            .left_join(leases::table.on(leases::id.nullable().eq(tenants::lease_id)))
            .left_join(rents::table.on(rents::lease_id.eq(leases::id)))
            .left_join(files::table.on(files::id.nullable().eq(rents::receipt_id)))
            .filter(files::id.eq(receipt_id))
            .first(&self.0.get()?)?)
    }

    fn by_payment_id(&mut self, payment_id: &PaymentId) -> Result<Person> {
        Ok(persons::table
            .select(persons::all_columns)
            .left_join(tenants::table.on(tenants::person_id.eq(persons::id)))
            .left_join(leases::table.on(leases::id.nullable().eq(tenants::lease_id)))
            .left_join(rents::table.on(rents::lease_id.eq(leases::id)))
            .left_join(payments::table.on(payments::rent_id.eq(rents::id)))
            .filter(payments::id.eq(payment_id))
            .first(&self.0.get()?)?)
    }

    fn by_lease_id(&mut self, lease_id: &LeaseId) -> Result<Person> {
        Ok(persons::table
            .select(persons::all_columns)
            .left_join(tenants::table.on(tenants::person_id.eq(persons::id)))
            .left_join(leases::table.on(leases::id.nullable().eq(tenants::lease_id)))
            .filter(leases::id.eq(lease_id))
            .first(&self.0.get()?)?)
    }

    fn by_step_id(&mut self, step_id: &StepId) -> Result<Person> {
        Ok(persons::table
            .select(persons::all_columns)
            .left_join(candidacies::table.on(candidacies::person_id.eq(persons::id)))
            .left_join(workflows::table.on(workflows::workflowable_id.eq(candidacies::id))) // TODO: match workflowable
            .left_join(steps::table.on(steps::workflow_id.eq(workflows::id)))
            .filter(steps::id.eq(step_id))
            .first(&self.0.get()?)?)
    }

    fn by_tenant_id(&mut self, tenant_id: &TenantId) -> Result<Person> {
        Ok(persons::table
            .select(persons::all_columns)
            .left_join(tenants::table.on(tenants::person_id.eq(persons::id)))
            .filter(tenants::id.eq(tenant_id))
            .first(&self.0.get()?)?)
    }

    fn create(&mut self, data: &Person) -> Result<Person> {
        Ok(insert_into(persons::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn create_many(&mut self, data: &[Person]) -> Result<Vec<Person>> {
        Ok(insert_into(persons::table)
            .values(data)
            .get_results(&self.0.get()?)?)
    }

    fn update(&mut self, data: &Person) -> Result<Person> {
        Ok(update(data).set(data).get_result(&self.0.get()?)?)
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

    fn by_id_with_balance(&mut self, id: &TenantId) -> Result<TenantWithBalance> {
        Ok(tenants::table
            .left_join(balances::table.on(balances::tenant_id.eq(tenants::id)))
            .select((tenants::all_columns, balances::all_columns.nullable()))
            .filter(tenants::id.eq(id))
            .first(&self.0.get()?)?)
    }

    fn by_auth_id_with_balances(&mut self, _auth_id: &AuthId) -> Result<Vec<TenantWithBalance>> {
        todo!()
        // Ok(tenants::table
        //     .left_join(persons::table.on(persons::account_id.eq(tenants::account_id)))
        //     .left_join(balances::table.on(balances::tenant_id.eq(tenants::id)))
        //     .select((tenants::all_columns, balances::all_columns.nullable()))
        //     .filter(persons::auth_id.eq(auth_id.inner()))
        //     .load(&self.0.get()?)?)
    }

    fn by_lease_id(&mut self, lease_id: &LeaseId) -> Result<Vec<Tenant>> {
        Ok(tenants::table
            .filter(tenants::lease_id.eq(lease_id))
            .load(&self.0.get()?)?)
    }

    fn by_person_id(&mut self, person_id: &PersonId) -> Result<Tenant> {
        Ok(tenants::table
            .filter(tenants::person_id.eq(person_id))
            .first(&self.0.get()?)?)
    }

    fn create(&mut self, data: &Tenant) -> Result<Tenant> {
        Ok(insert_into(tenants::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn create_many(&mut self, data: &[Tenant]) -> Result<Vec<Tenant>> {
        Ok(insert_into(tenants::table)
            .values(data)
            .get_results(&self.0.get()?)?)
    }

    fn delete(&mut self, data: TenantId) -> Result<Executed> {
        Ok(delete(tenants::table)
            .filter(tenants::id.eq(data))
            .execute(&self.0.get()?)?)
    }

    fn update(&mut self, data: &Tenant) -> Result<Tenant> {
        Ok(update(data).set(data).get_result(&self.0.get()?)?)
    }
}

impl database::WarrantStore for WarrantStore<'_> {
    fn with_identity(&mut self, warrant: Warrant) -> Result<WarrantWithIdentity> {
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
    }

    fn by_id(&mut self, id: &WarrantId) -> Result<WarrantWithIdentity> {
        self.with_identity(warrants::table.find(id).first(&self.0.get()?)?)
    }

    fn by_candidacy_id(&mut self, candidacy_id: &CandidacyId) -> Result<Vec<WarrantWithIdentity>> {
        warrants::table
            .filter(warrants::candidacy_id.eq(candidacy_id))
            .load::<Warrant>(&self.0.get()?)?
            .into_iter()
            .map(|warrant| self.with_identity(warrant))
            .collect()
    }

    fn by_tenant_id(&mut self, tenant_id: &TenantId) -> Result<Vec<WarrantWithIdentity>> {
        warrants::table
            .filter(warrants::tenant_id.eq(tenant_id))
            .load::<Warrant>(&self.0.get()?)?
            .into_iter()
            .map(|warrant| self.with_identity(warrant))
            .collect()
    }

    fn create(&mut self, data: &WarrantWithIdentity) -> Result<WarrantWithIdentity> {
        match (data.0.type_, data.1.clone()) {
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

    fn create_many(&mut self, data: &[WarrantWithIdentity]) -> Result<Vec<WarrantWithIdentity>> {
        data.iter().map(|warrant| self.create(warrant)).collect()
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

    fn by_account_id(&mut self, account_id: &AccountId) -> Result<Vec<LenderWithIdentity>> {
        lenders::table
            .select(lenders::all_columns)
            .left_join(persons::table.on(persons::account_id.eq(lenders::account_id)))
            .filter(lenders::account_id.eq(account_id))
            .load(&self.0.get()?)?
            .iter()
            .map(|lender: &Lender| self.by_id(&lender.id))
            .collect::<Result<Vec<_>>>()
    }

    fn by_account_id_first(&mut self, account_id: &AccountId) -> Result<LenderWithIdentity> {
        Ok(self
            .by_account_id(account_id)?
            .first()
            .cloned()
            .ok_or(NotFound)?)
    }

    fn by_individual_id(&mut self, individual_id: &PersonId) -> Result<LenderWithIdentity> {
        let lender: Lender = lenders::table
            .filter(lenders::individual_id.eq(individual_id))
            .first(&self.0.get()?)?;
        let person = persons::table.find(individual_id).first(&self.0.get()?)?;
        Ok((lender, LegalIdentity::Individual(person)))
    }

    fn create(&mut self, data: &Lender) -> Result<Lender> {
        Ok(insert_into(lenders::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: &Lender) -> Result<Lender> {
        Ok(update(data).set(data).get_result(&self.0.get()?)?)
    }
}

impl database::AdvertisementStore for AdvertisementStore<'_> {
    fn by_id(&mut self, id: &AdvertisementId) -> Result<Advertisement> {
        Ok(advertisements::table.find(id).first(&self.0.get()?)?)
    }

    fn by_id_published(&mut self, id: &AdvertisementId) -> Result<Advertisement> {
        Ok(advertisements::table
            .find(id)
            .filter(advertisements::published.eq(true)) // not found if published=false
            .first(&self.0.get()?)?)
    }

    fn by_candidacy_id(&mut self, candidacy_id: &CandidacyId) -> Result<Advertisement> {
        Ok(advertisements::table
            .select(advertisements::all_columns)
            .left_join(candidacies::table.on(candidacies::advertisement_id.eq(advertisements::id)))
            .filter(candidacies::id.eq(candidacy_id))
            .first(&self.0.get()?)?)
    }

    fn by_property_id(&mut self, property_id: &PropertyId) -> Result<Vec<Advertisement>> {
        Ok(advertisements::table
            .filter(advertisements::property_id.eq(property_id))
            .load(&self.0.get()?)?)
    }

    fn create(&mut self, data: &Advertisement) -> Result<Advertisement> {
        Ok(insert_into(advertisements::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: &Advertisement) -> Result<Advertisement> {
        Ok(update(data).set(data).get_result(&self.0.get()?)?)
    }
}

impl database::CandidacyStore for CandidacyStore<'_> {
    fn by_id(&mut self, id: &CandidacyId) -> Result<Candidacy> {
        Ok(candidacies::table.find(id).first(&self.0.get()?)?)
    }

    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Candidacy>> {
        Ok(candidacies::table
            .select(candidacies::all_columns)
            .left_join(
                advertisements::table.on(advertisements::id.eq(candidacies::advertisement_id)),
            )
            .left_join(properties::table.on(properties::id.eq(advertisements::property_id)))
            .left_join(accounts::table.on(accounts::id.eq(properties::account_id)))
            .left_join(persons::table.on(persons::account_id.eq(accounts::id)))
            .filter(persons::auth_id.eq(auth_id.inner()))
            .load(&self.0.get()?)?)
    }

    fn by_advertisement_id(
        &mut self,
        advertisement_id: &AdvertisementId,
    ) -> Result<Vec<Candidacy>> {
        Ok(candidacies::table
            .select(candidacies::all_columns)
            .left_join(
                advertisements::table.on(advertisements::id.eq(candidacies::advertisement_id)),
            )
            .filter(advertisements::id.eq(advertisement_id))
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

    fn by_person_id(&mut self, person_id: &PersonId) -> Result<Candidacy> {
        Ok(candidacies::table
            .select(candidacies::all_columns)
            .left_join(persons::table.on(persons::id.eq(candidacies::person_id)))
            .filter(persons::id.eq(person_id))
            .first(&self.0.get()?)?)
    }

    fn create(&mut self, data: &Candidacy) -> Result<Candidacy> {
        Ok(insert_into(candidacies::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: &Candidacy) -> Result<Candidacy> {
        Ok(update(data).set(data).get_result(&self.0.get()?)?)
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

    fn by_advertisement_id(&mut self, advertisement_id: &AdvertisementId) -> Result<Property> {
        Ok(properties::table
            .select(properties::all_columns)
            .left_join(advertisements::table.on(advertisements::property_id.eq(properties::id)))
            .filter(advertisements::id.eq(advertisement_id))
            .first(&self.0.get()?)?)
    }

    fn create(&mut self, data: &Property) -> Result<Property> {
        Ok(insert_into(properties::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn delete(&mut self, data: PropertyId) -> Result<Executed> {
        Ok(delete(properties::table)
            .filter(properties::id.eq(data))
            .execute(&self.0.get()?)?)
    }

    fn update(&mut self, data: &Property) -> Result<Property> {
        Ok(update(data).set(data).get_result(&self.0.get()?)?)
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

    fn by_lease_file_id(&mut self, lease_file_id: &LeaseFileId) -> Result<Lease> {
        Ok(leases::table
            .filter(leases::lease_id.eq(lease_file_id))
            .first(&self.0.get()?)?)
    }

    fn by_receipt_id(&mut self, receipt_id: &ReceiptId) -> Result<Lease> {
        Ok(leases::table
            .select(leases::all_columns)
            .left_join(rents::table.on(rents::lease_id.eq(leases::id)))
            .filter(rents::receipt_id.eq(receipt_id))
            .first(&self.0.get()?)?)
    }

    fn by_notice_id(&mut self, notice_id: &NoticeId) -> Result<Lease> {
        Ok(leases::table
            .select(leases::all_columns)
            .left_join(rents::table.on(rents::lease_id.eq(leases::id)))
            .filter(rents::notice_id.eq(notice_id))
            .first(&self.0.get()?)?)
    }

    fn by_rent_id(&mut self, rent_id: &RentId) -> Result<Lease> {
        Ok(leases::table
            .select(leases::all_columns)
            .left_join(rents::table.on(rents::lease_id.eq(leases::id)))
            .filter(rents::id.eq(rent_id))
            .first(&self.0.get()?)?)
    }

    fn by_tenant_id(&mut self, tenant_id: &TenantId) -> Result<Lease> {
        Ok(leases::table
            .select(leases::all_columns)
            .left_join(tenants::table.on(tenants::lease_id.eq(leases::id.nullable())))
            .filter(tenants::id.eq(tenant_id))
            .first(&self.0.get()?)?)
    }

    fn by_person_id(&mut self, person_id: &PersonId) -> Result<Lease> {
        Ok(leases::table
            .select(leases::all_columns)
            .left_join(tenants::table.on(tenants::lease_id.eq(leases::id.nullable())))
            .left_join(persons::table.on(persons::id.eq(tenants::person_id)))
            .filter(persons::id.eq(person_id))
            .first(&self.0.get()?)?)
    }

    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Lease>> {
        Ok(leases::table
            .select(leases::all_columns)
            .left_join(persons::table.on(persons::account_id.eq(leases::account_id)))
            .filter(persons::auth_id.eq(auth_id.inner()))
            .load(&self.0.get()?)?)
    }

    fn create(&mut self, data: &Lease) -> Result<Lease> {
        Ok(insert_into(leases::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn delete(&mut self, data: LeaseId) -> Result<Executed> {
        Ok(delete(leases::table)
            .filter(leases::id.eq(data))
            .execute(&self.0.get()?)?)
    }

    fn update(&mut self, data: &Lease) -> Result<Lease> {
        Ok(update(data).set(data).get_result(&self.0.get()?)?)
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
            .filter(rents::receipt_id.eq(receipt_id))
            .first(&self.0.get()?)?)
    }

    fn by_notice_id(&mut self, notice_id: &NoticeId) -> Result<Rent> {
        Ok(rents::table
            .filter(rents::notice_id.eq(notice_id))
            .first(&self.0.get()?)?)
    }

    fn by_lease_id(&mut self, lease_id: &LeaseId) -> Result<Vec<Rent>> {
        Ok(rents::table
            .filter(rents::lease_id.eq(lease_id))
            .load(&self.0.get()?)?)
    }

    fn create_many(&mut self, data: &[Rent]) -> Result<Vec<Rent>> {
        Ok(insert_into(rents::table)
            .values(data)
            .get_results(&self.0.get()?)?)
    }

    fn update(&mut self, data: &Rent) -> Result<Rent> {
        Ok(update(data).set(data).get_result(&self.0.get()?)?)
    }
}

impl database::FileStore for FileStore<'_> {
    fn by_external_id(&mut self, external_id: &ExternalId) -> Result<File> {
        Ok(files::table
            .filter(files::external_id.eq(external_id))
            .first(&self.0.get()?)?)
    }

    fn by_id(&mut self, id: &FileId) -> Result<File> {
        Ok(files::table.find(id).first(&self.0.get()?)?)
    }

    fn create(&mut self, data: &File) -> Result<File> {
        Ok(insert_into(files::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: &File) -> Result<File> {
        Ok(update(data).set(data).get_result(&self.0.get()?)?)
    }
}

impl database::PaymentStore for PaymentStore<'_> {
    fn create(&mut self, data: &Payment) -> Result<Payment> {
        Ok(insert_into(payments::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn by_rent_id(&mut self, rent_id: &RentId) -> Result<Vec<Payment>> {
        Ok(payments::table
            .select(payments::all_columns)
            .left_join(rents::table.on(rents::id.eq(payments::rent_id)))
            .filter(rents::id.eq(rent_id))
            .load(&self.0.get()?)?)
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
        let event = events::table
            .left_join(eventables::table.on(eventables::id.eq(events::eventable_id)))
            .left_join(files::table.on(files::id.nullable().eq(eventables::file_id)))
            .left_join(rents::table.on(rents::id.nullable().eq(eventables::rent_id)))
            .left_join(steps::table.on(steps::id.nullable().eq(eventables::step_id)))
            .left_join(leases::table.on(leases::id.nullable().eq(eventables::lease_id)))
            .left_join(payments::table.on(payments::id.nullable().eq(eventables::payment_id)))
            .left_join(
                candidacies::table.on(candidacies::id.nullable().eq(eventables::candidacy_id)),
            )
            .select((
                events::all_columns,
                files::all_columns.nullable(),
                rents::all_columns.nullable(),
                steps::all_columns.nullable(),
                leases::all_columns.nullable(),
                payments::all_columns.nullable(),
                candidacies::all_columns.nullable(),
            ))
            .filter(events::id.eq(id))
            .first::<EventableRow>(&self.0.get()?)?;
        Ok((event.0.clone(), event.into()))
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

    fn create(&mut self, data: &Event) -> Result<Event> {
        Ok(insert_into(events::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }
}

impl database::EventableStore for EventableStore<'_> {
    fn create(&mut self, data: &Eventable) -> Result<Eventable> {
        match &data {
            Eventable::File(inner) => insert_into(eventables::table)
                .values(eventables::file_id.eq(inner.id))
                .execute(&self.0.get()?)?,
            Eventable::Rent(inner) => insert_into(eventables::table)
                .values(eventables::rent_id.eq(inner.id))
                .execute(&self.0.get()?)?,
            Eventable::Step(inner) => insert_into(eventables::table)
                .values(eventables::step_id.eq(inner.id))
                .execute(&self.0.get()?)?,
            Eventable::Lease(inner) => insert_into(eventables::table)
                .values(eventables::lease_id.eq(inner.id))
                .execute(&self.0.get()?)?,
            Eventable::Payment(inner) => insert_into(eventables::table)
                .values(eventables::payment_id.eq(inner.id))
                .execute(&self.0.get()?)?,
            Eventable::Candidacy(inner) => insert_into(eventables::table)
                .values(eventables::candidacy_id.eq(inner.id))
                .execute(&self.0.get()?)?,
        };
        Ok(data.clone())
    }
}

impl database::ReportStore for ReportStore<'_> {
    fn by_account_id(&mut self, account_id: &AccountId) -> Result<Summary> {
        Ok(reports::table
            .filter(reports::account_id.eq(account_id))
            .first(&self.0.get()?)?)
    }
}

impl database::DiscussionStore for DiscussionStore<'_> {
    fn by_id(&mut self, id: &DiscussionId) -> Result<Discussion> {
        Ok(discussions::table.find(id).first(&self.0.get()?)?)
    }

    fn by_auth_id(&mut self, auth_id: &AuthId) -> Result<Vec<Discussion>> {
        Ok(discussions::table
            .select(discussions::all_columns)
            .left_join(persons::table.on(persons::account_id.eq(discussions::account_id)))
            .filter(persons::auth_id.eq(auth_id.inner()))
            .order(discussions::updated_at.desc())
            .load(&self.0.get()?)?)
    }

    fn by_initiator_id(&mut self, person_id: &PersonId) -> Result<Discussion> {
        Ok(discussions::table
            .select(discussions::all_columns)
            .left_join(persons::table.on(persons::id.eq(discussions::initiator_id)))
            .filter(persons::id.eq(person_id))
            .first(&self.0.get()?)?)
    }

    fn by_candidacy_id(&mut self, candidacy_id: &CandidacyId) -> Result<Discussion> {
        Ok(discussions::table
            .select(discussions::all_columns)
            .left_join(persons::table.on(persons::id.eq(discussions::initiator_id)))
            .left_join(candidacies::table.on(candidacies::person_id.eq(persons::id)))
            .filter(candidacies::id.eq(candidacy_id))
            .first(&self.0.get()?)?)
    }

    fn create(&mut self, data: &Discussion) -> Result<Discussion> {
        Ok(insert_into(discussions::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn create_many(&mut self, data: &[Discussion]) -> Result<Vec<Discussion>> {
        Ok(insert_into(discussions::table)
            .values(data)
            .get_results(&self.0.get()?)?)
    }

    fn update(&mut self, data: &Discussion) -> Result<Discussion> {
        Ok(update(data).set(data).get_result(&self.0.get()?)?)
    }

    fn delete(&mut self, data: DiscussionId) -> Result<Executed> {
        Ok(delete(discussions::table)
            .filter(discussions::id.eq(data))
            .execute(&self.0.get()?)?)
    }

    fn related_items(&mut self, id: &DiscussionId) -> Result<Vec<DiscussionItem>> {
        Ok(discussions::table
            .left_join(persons::table.on(persons::id.eq(discussions::initiator_id)))
            .left_join(tenants::table.on(tenants::person_id.eq(persons::id)))
            .left_join(candidacies::table.on(candidacies::person_id.eq(persons::id)))
            .left_join(leases::table.on(leases::id.nullable().eq(tenants::lease_id)))
            .select((
                candidacies::all_columns.nullable(),
                leases::all_columns.nullable(),
            ))
            .filter(discussions::id.eq(id))
            .load::<DiscussionItemRow>(&self.0.get()?)?
            .into_iter()
            .filter_map(|row| match row {
                (Some(candidacy), Some(lease)) => Some(vec![candidacy.into(), lease.into()]),
                (Some(candidacy), _) => Some(vec![candidacy.into()]),
                (_, Some(lease)) => Some(vec![lease.into()]),
                _ => None,
            })
            .flatten()
            .collect())
    }

    fn touch(&mut self, data: DiscussionId) -> Result<Executed> {
        Ok(update(discussions::table)
            .filter(discussions::id.eq(data))
            .set(discussions::updated_at.eq(now))
            .execute(&self.0.get()?)?)
    }
}

impl database::MessageStore for MessageStore<'_> {
    fn by_discussion_id(&mut self, discussion_id: &DiscussionId) -> Result<Vec<Message>> {
        Ok(messages::table
            .filter(messages::discussion_id.eq(discussion_id))
            .order(messages::created_at.desc())
            .load(&self.0.get()?)?)
    }

    fn create(&mut self, data: &Message) -> Result<Message> {
        Ok(insert_into(messages::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn create_many(&mut self, data: &[Message]) -> Result<Vec<Message>> {
        Ok(insert_into(messages::table)
            .values(data)
            .get_results(&self.0.get()?)?)
    }
}

impl database::InviteStore for InviteStore<'_> {
    fn by_token(&mut self, token: &InviteToken) -> Result<Invite> {
        Ok(invites::table
            .filter(invites::token.eq(token))
            .first(&self.0.get()?)?)
    }

    fn create(&mut self, data: &Invite) -> Result<Invite> {
        Ok(insert_into(invites::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: &Invite) -> Result<Invite> {
        Ok(update(data).set(data).get_result(&self.0.get()?)?)
    }
}

impl database::WorkflowableStore for WorkflowableStore<'_> {
    fn create(&mut self, data: &Workflowable) -> Result<Workflowable> {
        match &data {
            Workflowable::Candidacy(candidacy) => insert_into(workflowables::table)
                .values(workflowables::candidacy_id.eq(candidacy.id))
                .execute(&self.0.get()?)?,
        };
        Ok(data.clone())
    }
}

impl database::WorkflowStore for WorkflowStore<'_> {
    fn by_workflowable_id(
        &mut self,
        workflowable_id: &WorkflowableId,
    ) -> Result<Option<WorkflowWithSteps>> {
        let workflow = workflows::table
            .select(workflows::all_columns)
            .filter(workflows::workflowable_id.eq(workflowable_id))
            .first::<Workflow>(&self.0.get()?)
            .optional()?;

        let workflow = match workflow {
            Some(workflow) => workflow,
            None => return Ok(None),
        };

        let steps = Step::belonging_to(&workflow)
            .order(steps::created_at.asc())
            .load(&self.0.get()?)?;
        Ok(Some((workflow, steps)))
    }

    fn create(&mut self, data: &Workflow) -> Result<Workflow> {
        Ok(insert_into(workflows::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }
}

impl database::StepStore for StepStore<'_> {
    fn by_id(&mut self, id: &StepId) -> Result<Step> {
        Ok(steps::table.find(id).first(&self.0.get()?)?)
    }

    fn by_workflow_id(&mut self, workflow_id: &WorkflowId) -> Result<Vec<Step>> {
        Ok(steps::table
            .filter(steps::workflow_id.eq(workflow_id))
            .order(steps::created_at.asc())
            .load(&self.0.get()?)?)
    }

    fn create(&mut self, data: &Step) -> Result<Step> {
        Ok(insert_into(steps::table)
            .values(data)
            .get_result(&self.0.get()?)?)
    }

    fn update(&mut self, data: &Step) -> Result<Step> {
        Ok(update(data).set(data).get_result(&self.0.get()?)?)
    }
}
