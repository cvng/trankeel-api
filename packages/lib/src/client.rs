use crate::auth::CreateUserWithAccountInput;
use crate::auth::CreateUserWithAccountPayload;
use crate::auth::SignupUserFromInviteInput;
use crate::candidacies::AcceptCandidacyInput;
use crate::candidacies::CreateCandidacy;
use crate::candidacies::CreateCandidacyInput;
use crate::error::Result;
use crate::leases::AddExistingLease;
use crate::leases::AddExistingLeasePayload;
use crate::leases::CreateFurnishedLeaseInput;
use crate::leases::CreateNoticesInput;
use crate::leases::CreateReceiptsInput;
use crate::leases::DeleteLeaseInput;
use crate::leases::SendReceiptsInput;
use crate::leases::UpdateFurnishedLeaseInput;
use crate::messaging::DeleteDiscussionInput;
use crate::messaging::PushMessage;
use crate::messaging::PushMessageInput;
use crate::owners::UpdateIndividualLenderInput;
use crate::properties::CreateAdvertisementInput;
use crate::properties::CreateProperty;
use crate::properties::CreatePropertyInput;
use crate::properties::CreatePropertyPayload;
use crate::properties::DeletePropertyInput;
use crate::properties::UpdateAdvertisementInput;
use crate::properties::UpdatePropertyInput;
use crate::tenants::CreateTenant;
use crate::tenants::CreateTenantInput;
use crate::tenants::CreateTenantPayload;
use crate::tenants::DeleteTenantInput;
use crate::tenants::UpdateTenant;
use crate::tenants::UpdateTenantInput;
use crate::tenants::UpdateTenantPayload;
use crate::workflows::CompleteStepInput;
use crate::AddExistingLeaseInput;
use crate::PushMessagePayload;
use trankeel_core::context;
use trankeel_core::database::AccountStore;
use trankeel_core::database::AdvertisementStore;
use trankeel_core::database::CandidacyStore;
use trankeel_core::database::CompanyStore;
use trankeel_core::database::Db;
use trankeel_core::database::DiscussionStore;
use trankeel_core::database::EventStore;
use trankeel_core::database::FileStore;
use trankeel_core::database::LeaseStore;
use trankeel_core::database::LenderStore;
use trankeel_core::database::MessageStore;
use trankeel_core::database::PaymentStore;
use trankeel_core::database::PersonStore;
use trankeel_core::database::PlanStore;
use trankeel_core::database::PropertyStore;
use trankeel_core::database::RentStore;
use trankeel_core::database::ReportStore;
use trankeel_core::database::TenantStore;
use trankeel_core::database::WarrantStore;
use trankeel_core::database::WorkflowStore;
use trankeel_core::dispatcher::Command;
use trankeel_core::mailer::IntoMail;
use trankeel_core::mailer::Mail;
use trankeel_core::mailer::Mailer;
use trankeel_core::providers;
use trankeel_core::providers::Pdfmonkey;
use trankeel_core::providers::Pg;
use trankeel_core::providers::Sendinblue;
use trankeel_core::providers::Stripe;
use trankeel_data::Advertisement;
use trankeel_data::AuthId;
use trankeel_data::Candidacy;
use trankeel_data::DiscussionId;
use trankeel_data::Lease;
use trankeel_data::LeaseId;
use trankeel_data::Lender;
use trankeel_data::Notice;
use trankeel_data::Person;
use trankeel_data::Property;
use trankeel_data::PropertyId;
use trankeel_data::Receipt;
use trankeel_data::Step;
use trankeel_data::TenantId;

pub struct Client(context::Context);

impl<'a> Client {
    pub fn new(pg: Pg, pdfmonkey: Pdfmonkey, sendinblue: Sendinblue, stripe: Stripe) -> Self {
        Self(context::Context::new(pg, pdfmonkey, sendinblue, stripe))
    }

    // Stores

    pub fn accounts(&self) -> Box<dyn AccountStore + '_> {
        self.0.db().accounts()
    }

    pub fn persons(&self) -> Box<dyn PersonStore + '_> {
        self.0.db().persons()
    }

    pub fn companies(&self) -> Box<dyn CompanyStore + '_> {
        self.0.db().companies()
    }

    pub fn lenders(&self) -> Box<dyn LenderStore + '_> {
        self.0.db().lenders()
    }

    pub fn tenants(&self) -> Box<dyn TenantStore + '_> {
        self.0.db().tenants()
    }

    pub fn warrants(&self) -> Box<dyn WarrantStore + '_> {
        self.0.db().warrants()
    }

    pub fn advertisements(&self) -> Box<dyn AdvertisementStore + '_> {
        self.0.db().advertisements()
    }

    pub fn candidacies(&self) -> Box<dyn CandidacyStore + '_> {
        self.0.db().candidacies()
    }

    pub fn properties(&self) -> Box<dyn PropertyStore + '_> {
        self.0.db().properties()
    }

    pub fn leases(&self) -> Box<dyn LeaseStore + '_> {
        self.0.db().leases()
    }

    pub fn rents(&self) -> Box<dyn RentStore + '_> {
        self.0.db().rents()
    }

    pub fn files(&self) -> Box<dyn FileStore + '_> {
        self.0.db().files()
    }

    pub fn payments(&self) -> Box<dyn PaymentStore + '_> {
        self.0.db().payments()
    }

    pub fn plans(&self) -> Box<dyn PlanStore + '_> {
        self.0.db().plans()
    }

    pub fn events(&self) -> Box<dyn EventStore + '_> {
        self.0.db().events()
    }

    pub fn reports(&self) -> Box<dyn ReportStore + '_> {
        self.0.db().reports()
    }

    pub fn discussions(&self) -> Box<dyn DiscussionStore + '_> {
        self.0.db().discussions()
    }

    pub fn messages(&self) -> Box<dyn MessageStore + '_> {
        self.0.db().messages()
    }

    pub fn workflows(&self) -> Box<dyn WorkflowStore + '_> {
        self.0.db().workflows()
    }

    // Operations

    pub async fn create_user_with_account(
        &self,
        input: CreateUserWithAccountInput,
    ) -> Result<CreateUserWithAccountPayload> {
        crate::auth::create_user_with_account(self.0.db(), self.0.billing_provider(), input).await
    }

    pub async fn signup_user_from_invite(
        &self,
        input: SignupUserFromInviteInput,
    ) -> Result<Person> {
        crate::auth::signup_user_from_invite(self.0.db(), input).await
    }

    pub async fn create_candidacy(&self, input: CreateCandidacyInput) -> Result<Candidacy> {
        Ok(CreateCandidacy::new(&self.0).run(input).await?.candidacy)
    }

    pub async fn accept_candidacy(
        &self,
        auth_id: &AuthId,
        input: AcceptCandidacyInput,
    ) -> Result<Candidacy> {
        crate::candidacies::accept_candidacy(&self.0, auth_id, input).await
    }

    pub async fn create_tenant(
        &self,
        auth_id: &AuthId,
        input: CreateTenantInput,
    ) -> Result<CreateTenantPayload> {
        CreateTenant::new(&self.0, auth_id).run(input).await
    }

    pub async fn update_tenant(
        &self,
        _auth_id: &AuthId,
        input: UpdateTenantInput,
    ) -> Result<UpdateTenantPayload> {
        UpdateTenant::new(&self.0).run(input).await
    }

    pub fn delete_tenant(&self, auth_id: &AuthId, input: DeleteTenantInput) -> Result<TenantId> {
        crate::tenants::delete_tenant(self.0.db(), auth_id, input)
    }

    pub async fn create_property(
        &self,
        auth_id: &AuthId,
        input: CreatePropertyInput,
    ) -> Result<CreatePropertyPayload> {
        CreateProperty::new(&self.0, auth_id).run(input).await
    }

    pub fn update_property(
        &self,
        auth_id: &AuthId,
        input: UpdatePropertyInput,
    ) -> Result<Property> {
        crate::properties::update_property(self.0.db(), auth_id, input)
    }

    pub fn delete_property(
        &self,
        auth_id: &AuthId,
        input: DeletePropertyInput,
    ) -> Result<PropertyId> {
        crate::properties::delete_property(self.0.db(), auth_id, input)
    }

    pub fn create_advertisement(
        &self,
        auth_id: &AuthId,
        input: CreateAdvertisementInput,
    ) -> Result<Advertisement> {
        crate::properties::create_advertisement(self.0.db(), auth_id, input)
    }

    pub fn update_advertisement(
        &self,
        auth_id: &AuthId,
        input: UpdateAdvertisementInput,
    ) -> Result<Advertisement> {
        crate::properties::update_advertisement(self.0.db(), auth_id, input)
    }

    pub async fn add_existing_lease(
        &self,
        auth_id: &AuthId,
        input: AddExistingLeaseInput,
    ) -> Result<AddExistingLeasePayload> {
        AddExistingLease::new(&self.0, auth_id).run(input).await
    }

    pub fn create_furnished_lease(
        &self,
        auth_id: &AuthId,
        input: CreateFurnishedLeaseInput,
    ) -> Result<Lease> {
        crate::leases::create_furnished_lease(self.0.db(), auth_id, input)
    }

    pub fn update_furnished_lease(
        &self,
        auth_id: &AuthId,
        input: UpdateFurnishedLeaseInput,
    ) -> Result<Lease> {
        crate::leases::update_furnished_lease(self.0.db(), auth_id, input)
    }

    pub fn delete_lease(&self, auth_id: &AuthId, input: DeleteLeaseInput) -> Result<LeaseId> {
        crate::leases::delete_lease(self.0.db(), auth_id, input)
    }

    pub fn update_individual_lender(
        &self,
        auth_id: &AuthId,
        input: UpdateIndividualLenderInput,
    ) -> Result<Lender> {
        crate::owners::update_individual_lender(self.0.db(), auth_id, input)
    }

    pub async fn create_receipts(
        &self,
        auth_id: &AuthId,
        input: CreateReceiptsInput,
    ) -> Result<Vec<Receipt>> {
        crate::leases::create_receipts(self.0.db(), auth_id, self.0.pdfmaker(), input).await
    }

    pub async fn send_receipts(
        &self,
        auth_id: &AuthId,
        input: SendReceiptsInput,
    ) -> Result<Vec<Receipt>> {
        crate::leases::send_receipts(self.0.db(), auth_id, self.0.mailer(), input).await
    }

    pub async fn create_notices(
        &self,
        auth_id: &AuthId,
        input: CreateNoticesInput,
    ) -> Result<Vec<Notice>> {
        crate::leases::create_notices(self.0.db(), auth_id, self.0.pdfmaker(), input).await
    }

    pub fn delete_discussion(
        &self,
        auth_id: &AuthId,
        input: DeleteDiscussionInput,
    ) -> Result<DiscussionId> {
        crate::messaging::delete_discussion(self.0.db(), auth_id, input)
    }

    pub async fn push_message(&self, input: PushMessageInput) -> Result<PushMessagePayload> {
        PushMessage::new(&self.0).run(input).await
    }

    pub fn complete_step(&self, input: CompleteStepInput) -> Result<Step> {
        crate::workflows::complete_step(self.0.db(), input)
    }

    pub async fn batch_mails(&self, mails: Vec<impl IntoMail>) -> Result<Vec<Mail>> {
        self.0.mailer().batch(mails).await
    }
}

pub fn init() -> Result<Client> {
    Ok(Client::new(
        providers::Pg::init(),
        providers::Pdfmonkey::init(),
        providers::Sendinblue::init(),
        providers::Stripe::init(),
    ))
}
