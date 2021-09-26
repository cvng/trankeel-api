pub use crate::auth::AccountActivatePlanInput;
pub use crate::auth::AccountUpdateInput;
pub use crate::auth::AddressInput;
pub use crate::auth::CreateUserWithAccountInput;
pub use crate::candidacies::AcceptCandidacyInput;
pub use crate::candidacies::CreateCandidacyInput;
pub use crate::error::Error;
pub use crate::files::CreateFileInput;
pub use crate::imports::ImportInput;
pub use crate::leases::CreateFurnishedLeaseInput;
pub use crate::leases::CreateReceiptsInput;
pub use crate::leases::DeleteLeaseInput;
pub use crate::leases::SendPaymentNoticeInput;
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

use crate::Advertisement;
use crate::AuthId;
use crate::Candidacy;
use crate::Lease;
use crate::LeaseId;
use crate::Lender;
use crate::Person;
use crate::Property;
use crate::PropertyId;
use crate::Receipt;
use crate::Tenant;
use crate::TenantId;
use async_graphql::Context;
use piteo_core::providers::Pdfmonkey;
use piteo_core::providers::Pg;
use piteo_core::providers::PgPool;
use piteo_core::providers::Provider;
use piteo_core::providers::Sendinblue;
use piteo_core::providers::Stripe;
use piteo_data::Summary;

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

    fn auth_id(&self) -> Result<AuthId, Error> {
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
) -> Result<Person, Error> {
    crate::auth::create_user_with_account(&client.db(), &Stripe::init(), input).await
}

// # Candidacies

pub fn create_candidacy(client: &Client, input: CreateCandidacyInput) -> Result<Candidacy, Error> {
    crate::candidacies::create_candidacy(&client.db(), input)
}

pub fn accept_candidacy(client: &Client, input: AcceptCandidacyInput) -> Result<Candidacy, Error> {
    crate::candidacies::accept_candidacy(&client.db(), &client.auth_id()?, input)
}

// # Tenants

pub fn create_tenant(client: &Client, input: CreateTenantInput) -> Result<Tenant, Error> {
    crate::tenants::create_tenant(&client.db(), &client.auth_id()?, input, None)
}

pub fn update_tenant(client: &Client, input: UpdateTenantInput) -> Result<Tenant, Error> {
    crate::tenants::update_tenant(&client.db(), &client.auth_id()?, input)
}

pub fn delete_tenant(client: &Client, id: TenantId) -> Result<TenantId, Error> {
    crate::tenants::delete_tenant(&client.db(), &client.auth_id()?, DeleteTenantInput { id })
}

// # Properties

pub fn create_property(client: &Client, input: CreatePropertyInput) -> Result<Property, Error> {
    crate::properties::create_property(&client.db(), &client.auth_id()?, input)
}

pub fn update_property(client: &Client, input: UpdatePropertyInput) -> Result<Property, Error> {
    crate::properties::update_property(&client.db(), &client.auth_id()?, input)
}

pub fn delete_property(client: &Client, id: PropertyId) -> Result<TenantId, Error> {
    crate::properties::delete_property(&client.db(), &client.auth_id()?, DeletePropertyInput { id })
}

// # Avertisements

pub fn create_advertisement(
    client: &Client,
    input: CreateAdvertisementInput,
) -> Result<Advertisement, Error> {
    crate::properties::create_advertisement(&client.db(), &client.auth_id()?, input)
}

// # Leases

pub fn create_furnished_lease(
    client: &Client,
    input: CreateFurnishedLeaseInput,
) -> Result<Lease, Error> {
    crate::leases::create_furnished_lease(&client.db(), &client.auth_id()?, input)
}

pub fn update_furnished_lease(
    client: &Client,
    input: UpdateFurnishedLeaseInput,
) -> Result<Lease, Error> {
    crate::leases::update_furnished_lease(&client.db(), &client.auth_id()?, input)
}

pub fn delete_lease(client: &Client, id: LeaseId) -> Result<LeaseId, Error> {
    crate::leases::delete_lease(&client.db(), &client.auth_id()?, DeleteLeaseInput { id })
}

// # Lenders

pub fn update_individual_lender(
    client: &Client,
    input: UpdateIndividualLenderInput,
) -> Result<Lender, Error> {
    crate::owners::update_individual_lender(&client.db(), &client.auth_id()?, input)
}

// # Receipts

pub async fn create_receipts(
    client: &Client,
    input: CreateReceiptsInput,
) -> Result<Vec<Receipt>, Error> {
    crate::leases::create_receipts(&client.db(), &client.auth_id()?, &Pdfmonkey::init(), input)
        .await
}

pub async fn send_receipts(
    client: &Client,
    input: SendReceiptsInput,
) -> Result<Vec<Receipt>, Error> {
    crate::leases::send_receipts(&client.db(), &client.auth_id()?, &Sendinblue::init(), input).await
}

// # Reports

pub fn get_summary() -> Result<Summary, Error> {
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
