pub use crate::auth::AccountActivatePlanInput;
pub use crate::auth::AccountUpdateInput;
pub use crate::auth::AddressInput;
pub use crate::auth::CreateUserWithAccountInput;
pub use crate::candidacies::AcceptCandidacyInput;
pub use crate::candidacies::CreateCandidacyInput;
pub use crate::error::Error;
pub use crate::error::Result;
pub use crate::files::CreateFileInput;
pub use crate::imports::ImportInput;
pub use crate::leases::CreateFurnishedLeaseInput;
pub use crate::leases::CreateNoticesInput;
pub use crate::leases::CreateReceiptsInput;
pub use crate::leases::DeleteLeaseInput;
pub use crate::leases::SendReceiptsInput;
pub use crate::leases::TransactionInput;
pub use crate::leases::UpdateFurnishedLeaseInput;
pub use crate::owners::UpdateIndividualLenderInput;
pub use crate::properties::CreateAdvertisementInput;
pub use crate::properties::CreatePropertyInput;
pub use crate::properties::DeletePropertyInput;
pub use crate::properties::UpdatePropertyInput;
pub use crate::tenants::CreateTenantInput;
pub use crate::tenants::DeleteTenantInput;
pub use crate::tenants::UpdateTenantInput;
pub use piteo_core::database::Db;

use async_graphql::Context;
use piteo_core::providers::Pdfmonkey;
use piteo_core::providers::Pg;
use piteo_core::providers::PgPool;
use piteo_core::providers::Sendinblue;
use piteo_core::providers::Stripe;
use piteo_data::Advertisement;
use piteo_data::AuthId;
use piteo_data::Candidacy;
use piteo_data::Lease;
use piteo_data::LeaseId;
use piteo_data::Lender;
use piteo_data::PaymentNotice;
use piteo_data::Person;
use piteo_data::Property;
use piteo_data::PropertyId;
use piteo_data::Receipt;
use piteo_data::Summary;
use piteo_data::Tenant;
use piteo_data::TenantId;

// # Client

pub struct Client(PgPool, Option<AuthId>);

impl Client {
    pub fn new(db_pool: PgPool) -> Self {
        Self(db_pool, None)
    }

    pub fn set_db_pool(&mut self, db_pool: PgPool) {
        self.0 = db_pool
    }

    pub fn set_auth_id(&mut self, auth_id: AuthId) {
        self.1 = Some(auth_id);
    }

    fn db(&self) -> Pg {
        Pg::new(self.db_pool())
    }

    fn db_pool(&self) -> PgPool {
        self.0.clone()
    }

    fn auth_id(&self) -> Result<AuthId> {
        self.1.clone().ok_or_else(|| Error::msg("no auth id"))
    }
}

pub fn init() -> Client {
    Client(Pg::init().inner(), None)
}

// # Datasources

pub fn db(client: &Client) -> Pg {
    client.db()
}

// # Auth

pub async fn create_user_with_account(
    client: &Client,
    input: CreateUserWithAccountInput,
) -> Result<Person> {
    crate::auth::create_user_with_account(&client.db(), &Stripe::init(), input).await
}

// # Candidacies

pub fn create_candidacy(client: &Client, input: CreateCandidacyInput) -> Result<Candidacy> {
    crate::candidacies::create_candidacy(&client.db(), input)
}

pub fn accept_candidacy(client: &Client, input: AcceptCandidacyInput) -> Result<Candidacy> {
    crate::candidacies::accept_candidacy(&client.db(), &client.auth_id()?, input)
}

// # Tenants

pub fn create_tenant(client: &Client, input: CreateTenantInput) -> Result<Tenant> {
    crate::tenants::create_tenant(&client.db(), &client.auth_id()?, input, None)
}

pub fn update_tenant(client: &Client, input: UpdateTenantInput) -> Result<Tenant> {
    crate::tenants::update_tenant(&client.db(), &client.auth_id()?, input)
}

pub fn delete_tenant(client: &Client, input: DeleteTenantInput) -> Result<TenantId> {
    crate::tenants::delete_tenant(&client.db(), &client.auth_id()?, input)
}

// # Properties

pub fn create_property(client: &Client, input: CreatePropertyInput) -> Result<Property> {
    crate::properties::create_property(&client.db(), &client.auth_id()?, input)
}

pub fn update_property(client: &Client, input: UpdatePropertyInput) -> Result<Property> {
    crate::properties::update_property(&client.db(), &client.auth_id()?, input)
}

pub fn delete_property(client: &Client, input: DeletePropertyInput) -> Result<PropertyId> {
    crate::properties::delete_property(&client.db(), &client.auth_id()?, input)
}

// # Avertisements

pub fn create_advertisement(
    client: &Client,
    input: CreateAdvertisementInput,
) -> Result<Advertisement> {
    crate::properties::create_advertisement(&client.db(), &client.auth_id()?, input)
}

// # Leases

pub fn create_furnished_lease(client: &Client, input: CreateFurnishedLeaseInput) -> Result<Lease> {
    crate::leases::create_furnished_lease(&client.db(), &client.auth_id()?, input)
}

pub fn update_furnished_lease(client: &Client, input: UpdateFurnishedLeaseInput) -> Result<Lease> {
    crate::leases::update_furnished_lease(&client.db(), &client.auth_id()?, input)
}

pub fn delete_lease(client: &Client, input: DeleteLeaseInput) -> Result<LeaseId> {
    crate::leases::delete_lease(&client.db(), &client.auth_id()?, input)
}

// # Lenders

pub fn update_individual_lender(
    client: &Client,
    input: UpdateIndividualLenderInput,
) -> Result<Lender> {
    crate::owners::update_individual_lender(&client.db(), &client.auth_id()?, input)
}

// # Receipts

pub async fn create_receipts(client: &Client, input: CreateReceiptsInput) -> Result<Vec<Receipt>> {
    crate::leases::create_receipts(&client.db(), &client.auth_id()?, &Pdfmonkey::init(), input)
        .await
}

pub async fn send_receipts(client: &Client, input: SendReceiptsInput) -> Result<Vec<Receipt>> {
    crate::leases::send_receipts(&client.db(), &client.auth_id()?, &Sendinblue::init(), input).await
}

// # Notices

pub async fn create_notices(
    client: &Client,
    input: CreateNoticesInput,
) -> Result<Vec<PaymentNotice>> {
    crate::leases::create_notices(&client.db(), &client.auth_id()?, &Pdfmonkey::init(), input).await
}

// # Reports

pub fn get_summary() -> Result<Summary> {
    crate::reports::get_summary()
}

// # Utils

impl From<&Context<'_>> for Client {
    fn from(item: &Context<'_>) -> Self {
        Self(
            item.data_unchecked::<Client>().0.clone(),
            item.data_opt::<AuthId>().cloned(),
        )
    }
}
