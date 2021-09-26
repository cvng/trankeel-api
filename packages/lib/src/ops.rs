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
pub use piteo_core::database::Db;
pub use piteo_core::error::Error;
pub use piteo_core::providers::DbPool;
use piteo_core::providers::Pdfmonkey;
pub use piteo_core::providers::Pg;
pub use piteo_core::providers::Provider;
use piteo_core::providers::Sendinblue;
use piteo_core::providers::Stripe;
use piteo_data::Summary;

// # Datasources

pub fn db(db_pool: &DbPool) -> Pg {
    Pg::new(db_pool.clone())
}

// # Auth

pub async fn create_user_with_account(
    db_pool: &DbPool,
    input: CreateUserWithAccountInput,
) -> Result<Person, Error> {
    crate::auth::create_user_with_account(&Pg::new(db_pool.clone()), &Stripe::init(), input).await
}

// # Candidacies

pub fn create_candidacy(db_pool: &DbPool, input: CreateCandidacyInput) -> Result<Candidacy, Error> {
    crate::candidacies::create_candidacy(&Pg::new(db_pool.clone()), input)
}

pub fn accept_candidacy(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: AcceptCandidacyInput,
) -> Result<Candidacy, Error> {
    crate::candidacies::accept_candidacy(&Pg::new(db_pool.clone()), auth_id, input)
}

// # Tenants

pub fn create_tenant(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: CreateTenantInput,
) -> Result<Tenant, Error> {
    crate::tenants::create_tenant(&Pg::new(db_pool.clone()), auth_id, input, None)
}

pub fn update_tenant(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: UpdateTenantInput,
) -> Result<Tenant, Error> {
    crate::tenants::update_tenant(&Pg::new(db_pool.clone()), auth_id, input)
}

pub fn delete_tenant(db_pool: &DbPool, auth_id: &AuthId, id: TenantId) -> Result<TenantId, Error> {
    crate::tenants::delete_tenant(&Pg::new(db_pool.clone()), auth_id, DeleteTenantInput { id })
}

// # Properties

pub fn create_property(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: CreatePropertyInput,
) -> Result<Property, Error> {
    crate::properties::create_property(&Pg::new(db_pool.clone()), auth_id, input)
}

pub fn update_property(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: UpdatePropertyInput,
) -> Result<Property, Error> {
    crate::properties::update_property(&Pg::new(db_pool.clone()), auth_id, input)
}

pub fn delete_property(
    db_pool: &DbPool,
    auth_id: &AuthId,
    id: PropertyId,
) -> Result<TenantId, Error> {
    crate::properties::delete_property(
        &Pg::new(db_pool.clone()),
        auth_id,
        DeletePropertyInput { id },
    )
}

pub fn create_advertisement(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: CreateAdvertisementInput,
) -> Result<Advertisement, Error> {
    crate::properties::create_advertisement(&Pg::new(db_pool.clone()), auth_id, input)
}

// # Leases

pub fn create_furnished_lease(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: CreateFurnishedLeaseInput,
) -> Result<Lease, Error> {
    crate::leases::create_furnished_lease(&Pg::new(db_pool.clone()), auth_id, input)
}

pub fn update_furnished_lease(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: UpdateFurnishedLeaseInput,
) -> Result<Lease, Error> {
    crate::leases::update_furnished_lease(&Pg::new(db_pool.clone()), auth_id, input)
}

pub fn delete_lease(db_pool: &DbPool, auth_id: &AuthId, id: LeaseId) -> Result<LeaseId, Error> {
    crate::leases::delete_lease(&Pg::new(db_pool.clone()), auth_id, DeleteLeaseInput { id })
}

// # Lenders

pub fn update_individual_lender(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: UpdateIndividualLenderInput,
) -> Result<Lender, Error> {
    crate::owners::update_individual_lender(&Pg::new(db_pool.clone()), auth_id, input)
}

// # Receipts

pub async fn create_receipts(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: CreateReceiptsInput,
) -> Result<Vec<Receipt>, Error> {
    crate::leases::create_receipts(
        &Pg::new(db_pool.clone()),
        auth_id,
        &Pdfmonkey::init(),
        input,
    )
    .await
}

pub async fn send_receipts(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: SendReceiptsInput,
) -> Result<Vec<Receipt>, Error> {
    crate::leases::send_receipts(
        &Pg::new(db_pool.clone()),
        auth_id,
        &Sendinblue::init(),
        input,
    )
    .await
}

// # Reports

pub fn get_summary() -> Result<Summary, Error> {
    crate::reports::get_summary()
}
