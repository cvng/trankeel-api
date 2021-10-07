use crate::auth::CreateUserWithAccountInput;
use crate::candidacies::AcceptCandidacyInput;
use crate::candidacies::CreateCandidacyInput;
use crate::error::Result;
use crate::leases::CreateFurnishedLeaseInput;
use crate::leases::CreateNoticesInput;
use crate::leases::CreateReceiptsInput;
use crate::leases::DeleteLeaseInput;
use crate::leases::SendReceiptsInput;
use crate::leases::UpdateFurnishedLeaseInput;
use crate::messaging::DeleteDiscussionInput;
use crate::messaging::PushMessageInput;
use crate::owners::UpdateIndividualLenderInput;
use crate::properties::CreateAdvertisementInput;
use crate::properties::CreatePropertyInput;
use crate::properties::DeletePropertyInput;
use crate::properties::UpdateAdvertisementInput;
use crate::properties::UpdatePropertyInput;
use crate::tenants::CreateTenantInput;
use crate::tenants::DeleteTenantInput;
use crate::tenants::UpdateTenantInput;
use piteo_core::database::AccountStore;
use piteo_core::database::AdvertisementStore;
use piteo_core::database::CandidacyStore;
use piteo_core::database::CompanyStore;
use piteo_core::database::Db;
use piteo_core::database::DiscussionStore;
use piteo_core::database::EventStore;
use piteo_core::database::FileStore;
use piteo_core::database::LeaseStore;
use piteo_core::database::LenderStore;
use piteo_core::database::MessageStore;
use piteo_core::database::PaymentStore;
use piteo_core::database::PersonStore;
use piteo_core::database::PlanStore;
use piteo_core::database::PropertyStore;
use piteo_core::database::RentStore;
use piteo_core::database::ReportStore;
use piteo_core::database::TenantStore;
use piteo_core::database::WarrantStore;
use piteo_core::providers::Pdfmonkey;
use piteo_core::providers::Pg;
use piteo_core::providers::Sendinblue;
use piteo_core::providers::Stripe;
use piteo_data::Advertisement;
use piteo_data::AuthId;
use piteo_data::Candidacy;
use piteo_data::DiscussionId;
use piteo_data::Lease;
use piteo_data::LeaseId;
use piteo_data::Lender;
use piteo_data::Message;
use piteo_data::PaymentNotice;
use piteo_data::Person;
use piteo_data::Property;
use piteo_data::PropertyId;
use piteo_data::Receipt;
use piteo_data::Tenant;
use piteo_data::TenantId;

pub struct Client(Pg, Pdfmonkey, Sendinblue, Stripe);

impl<'a> Client {
    pub fn new(pg: Pg, pdfmonkey: Pdfmonkey, sendinblue: Sendinblue, stripe: Stripe) -> Self {
        Self(pg, pdfmonkey, sendinblue, stripe)
    }

    // Stores

    pub fn accounts(&self) -> Box<dyn AccountStore + '_> {
        self.0.accounts()
    }

    pub fn persons(&self) -> Box<dyn PersonStore + '_> {
        self.0.persons()
    }

    pub fn companies(&self) -> Box<dyn CompanyStore + '_> {
        self.0.companies()
    }

    pub fn lenders(&self) -> Box<dyn LenderStore + '_> {
        self.0.lenders()
    }

    pub fn tenants(&self) -> Box<dyn TenantStore + '_> {
        self.0.tenants()
    }

    pub fn warrants(&self) -> Box<dyn WarrantStore + '_> {
        self.0.warrants()
    }

    pub fn advertisements(&self) -> Box<dyn AdvertisementStore + '_> {
        self.0.advertisements()
    }

    pub fn candidacies(&self) -> Box<dyn CandidacyStore + '_> {
        self.0.candidacies()
    }

    pub fn properties(&self) -> Box<dyn PropertyStore + '_> {
        self.0.properties()
    }

    pub fn leases(&self) -> Box<dyn LeaseStore + '_> {
        self.0.leases()
    }

    pub fn rents(&self) -> Box<dyn RentStore + '_> {
        self.0.rents()
    }

    pub fn files(&self) -> Box<dyn FileStore + '_> {
        self.0.files()
    }

    pub fn payments(&self) -> Box<dyn PaymentStore + '_> {
        self.0.payments()
    }

    pub fn plans(&self) -> Box<dyn PlanStore + '_> {
        self.0.plans()
    }

    pub fn events(&self) -> Box<dyn EventStore + '_> {
        self.0.events()
    }

    pub fn reports(&self) -> Box<dyn ReportStore + '_> {
        self.0.reports()
    }

    pub fn discussions(&self) -> Box<dyn DiscussionStore + '_> {
        self.0.discussions()
    }

    pub fn messages(&self) -> Box<dyn MessageStore + '_> {
        self.0.messages()
    }

    // Operations

    pub async fn create_user_with_account(
        &self,
        input: CreateUserWithAccountInput,
    ) -> Result<Person> {
        crate::auth::create_user_with_account(&self.0, &self.3, input).await
    }

    pub fn create_candidacy(&self, input: CreateCandidacyInput) -> Result<Candidacy> {
        crate::candidacies::create_candidacy(&self.0, input)
    }

    pub fn accept_candidacy(
        &self,
        auth_id: &AuthId,
        input: AcceptCandidacyInput,
    ) -> Result<Candidacy> {
        crate::candidacies::accept_candidacy(&self.0, auth_id, input)
    }

    pub fn create_tenant(&self, auth_id: &AuthId, input: CreateTenantInput) -> Result<Tenant> {
        crate::tenants::create_tenant(&self.0, auth_id, input, None)
    }

    pub fn update_tenant(&self, auth_id: &AuthId, input: UpdateTenantInput) -> Result<Tenant> {
        crate::tenants::update_tenant(&self.0, auth_id, input)
    }

    pub fn delete_tenant(&self, auth_id: &AuthId, input: DeleteTenantInput) -> Result<TenantId> {
        crate::tenants::delete_tenant(&self.0, auth_id, input)
    }

    pub fn create_property(
        &self,
        auth_id: &AuthId,
        input: CreatePropertyInput,
    ) -> Result<Property> {
        crate::properties::create_property(&self.0, auth_id, input)
    }

    pub fn update_property(
        &self,
        auth_id: &AuthId,
        input: UpdatePropertyInput,
    ) -> Result<Property> {
        crate::properties::update_property(&self.0, auth_id, input)
    }

    pub fn delete_property(
        &self,
        auth_id: &AuthId,
        input: DeletePropertyInput,
    ) -> Result<PropertyId> {
        crate::properties::delete_property(&self.0, auth_id, input)
    }

    pub fn create_advertisement(
        &self,
        auth_id: &AuthId,
        input: CreateAdvertisementInput,
    ) -> Result<Advertisement> {
        crate::properties::create_advertisement(&self.0, auth_id, input)
    }

    pub fn update_advertisement(
        &self,
        auth_id: &AuthId,
        input: UpdateAdvertisementInput,
    ) -> Result<Advertisement> {
        crate::properties::update_advertisement(&self.0, auth_id, input)
    }

    pub fn create_furnished_lease(
        &self,
        auth_id: &AuthId,
        input: CreateFurnishedLeaseInput,
    ) -> Result<Lease> {
        crate::leases::create_furnished_lease(&self.0, auth_id, input)
    }

    pub fn update_furnished_lease(
        &self,
        auth_id: &AuthId,
        input: UpdateFurnishedLeaseInput,
    ) -> Result<Lease> {
        crate::leases::update_furnished_lease(&self.0, auth_id, input)
    }

    pub fn delete_lease(&self, auth_id: &AuthId, input: DeleteLeaseInput) -> Result<LeaseId> {
        crate::leases::delete_lease(&self.0, auth_id, input)
    }

    pub fn update_individual_lender(
        &self,
        auth_id: &AuthId,
        input: UpdateIndividualLenderInput,
    ) -> Result<Lender> {
        crate::owners::update_individual_lender(&self.0, auth_id, input)
    }

    pub async fn create_receipts(
        &self,
        auth_id: &AuthId,
        input: CreateReceiptsInput,
    ) -> Result<Vec<Receipt>> {
        crate::leases::create_receipts(&self.0, auth_id, &self.1, input).await
    }

    pub async fn send_receipts(
        &self,
        auth_id: &AuthId,
        input: SendReceiptsInput,
    ) -> Result<Vec<Receipt>> {
        crate::leases::send_receipts(&self.0, auth_id, &self.2, input).await
    }

    pub async fn create_notices(
        &self,
        auth_id: &AuthId,
        input: CreateNoticesInput,
    ) -> Result<Vec<PaymentNotice>> {
        crate::leases::create_notices(&self.0, auth_id, &self.1, input).await
    }

    pub fn delete_discussion(
        &self,
        auth_id: &AuthId,
        input: DeleteDiscussionInput,
    ) -> Result<DiscussionId> {
        crate::messaging::delete_discussion(&self.0, auth_id, input)
    }

    pub fn push_message(&self, input: PushMessageInput) -> Result<Message> {
        crate::messaging::push_message(&self.0, input)
    }
}

pub fn init() -> Result<Client> {
    Ok(Client::new(
        Pg::init(),
        Pdfmonkey::init(),
        Sendinblue::init(),
        Stripe::init(),
    ))
}
