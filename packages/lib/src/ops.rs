pub use crate::auth::AccountActivatePlanInput;
pub use crate::auth::AccountUpdateInput;
pub use crate::auth::AddressInput;
pub use crate::auth::CreateUserWithAccountInput;
pub use crate::candidacies::AcceptCandidacyInput;
pub use crate::candidacies::CreateCandidacyInput;
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
pub use piteo_core::error::Error;
pub use piteo_core::providers::Pg;
pub use piteo_core::providers::Provider;

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
use piteo_core::providers::PgPool;
use piteo_core::providers::Pdfmonkey;
use piteo_core::providers::Sendinblue;
use piteo_core::providers::Stripe;
use piteo_data::Summary;

pub struct Client(PgPool, AuthId);

impl Client {
    pub fn new(db_pool: PgPool) -> Self {
        Self(db_pool, AuthId::default())
    }

    pub fn with_auth_id(db_pool: PgPool, auth_id: AuthId) -> Self {
        Self(db_pool, auth_id)
    }
}

// # Datasources

pub fn db(client: &Client) -> Pg {
    Pg::new(client.0.clone())
}

// # Auth

pub async fn create_user_with_account(
    client: &Client,
    input: CreateUserWithAccountInput,
) -> Result<Person, Error> {
    crate::auth::create_user_with_account(&Pg::new(client.0.clone()), &Stripe::init(), input).await
}

// # Candidacies

pub fn create_candidacy(client: &Client, input: CreateCandidacyInput) -> Result<Candidacy, Error> {
    crate::candidacies::create_candidacy(&Pg::new(client.0.clone()), input)
}

pub fn accept_candidacy(client: &Client, input: AcceptCandidacyInput) -> Result<Candidacy, Error> {
    crate::candidacies::accept_candidacy(&Pg::new(client.0.clone()), &client.1, input)
}

// # Tenants

pub fn create_tenant(client: &Client, input: CreateTenantInput) -> Result<Tenant, Error> {
    crate::tenants::create_tenant(&Pg::new(client.0.clone()), &client.1, input, None)
}

pub fn update_tenant(client: &Client, input: UpdateTenantInput) -> Result<Tenant, Error> {
    crate::tenants::update_tenant(&Pg::new(client.0.clone()), &client.1, input)
}

pub fn delete_tenant(client: &Client, id: TenantId) -> Result<TenantId, Error> {
    crate::tenants::delete_tenant(
        &Pg::new(client.0.clone()),
        &client.1,
        DeleteTenantInput { id },
    )
}

// # Properties

pub fn create_property(client: &Client, input: CreatePropertyInput) -> Result<Property, Error> {
    crate::properties::create_property(&Pg::new(client.0.clone()), &client.1, input)
}

pub fn update_property(client: &Client, input: UpdatePropertyInput) -> Result<Property, Error> {
    crate::properties::update_property(&Pg::new(client.0.clone()), &client.1, input)
}

pub fn delete_property(client: &Client, id: PropertyId) -> Result<TenantId, Error> {
    crate::properties::delete_property(
        &Pg::new(client.0.clone()),
        &client.1,
        DeletePropertyInput { id },
    )
}

pub fn create_advertisement(
    client: &Client,
    input: CreateAdvertisementInput,
) -> Result<Advertisement, Error> {
    crate::properties::create_advertisement(&Pg::new(client.0.clone()), &client.1, input)
}

// # Leases

pub fn create_furnished_lease(
    client: &Client,
    input: CreateFurnishedLeaseInput,
) -> Result<Lease, Error> {
    crate::leases::create_furnished_lease(&Pg::new(client.0.clone()), &client.1, input)
}

pub fn update_furnished_lease(
    client: &Client,
    input: UpdateFurnishedLeaseInput,
) -> Result<Lease, Error> {
    crate::leases::update_furnished_lease(&Pg::new(client.0.clone()), &client.1, input)
}

pub fn delete_lease(client: &Client, id: LeaseId) -> Result<LeaseId, Error> {
    crate::leases::delete_lease(
        &Pg::new(client.0.clone()),
        &client.1,
        DeleteLeaseInput { id },
    )
}

// # Lenders

pub fn update_individual_lender(
    client: &Client,
    input: UpdateIndividualLenderInput,
) -> Result<Lender, Error> {
    crate::owners::update_individual_lender(&Pg::new(client.0.clone()), &client.1, input)
}

// # Receipts

pub async fn create_receipts(
    client: &Client,
    input: CreateReceiptsInput,
) -> Result<Vec<Receipt>, Error> {
    crate::leases::create_receipts(
        &Pg::new(client.0.clone()),
        &client.1,
        &Pdfmonkey::init(),
        input,
    )
    .await
}

pub async fn send_receipts(
    client: &Client,
    input: SendReceiptsInput,
) -> Result<Vec<Receipt>, Error> {
    crate::leases::send_receipts(
        &Pg::new(client.0.clone()),
        &client.1,
        &Sendinblue::init(),
        input,
    )
    .await
}

// # Reports

pub fn get_summary() -> Result<Summary, Error> {
    crate::reports::get_summary()
}

impl From<&Context<'_>> for Client {
    fn from(item: &Context<'_>) -> Self {
        Self(
            item.data_unchecked::<PgPool>().clone(),
            item.data_opt::<AuthId>().cloned().unwrap_or_default(),
        )
    }
}
