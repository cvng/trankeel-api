use crate::providers::DbPool;
use crate::providers::Pdfmonkey;
use crate::providers::Pg;
use crate::providers::Sendinblue;
use crate::providers::Stripe;
use crate::Provider;
use piteo_core::auth;
use piteo_core::auth::CreateUserWithAccountInput;
use piteo_core::error::Error;
use piteo_core::leases;
use piteo_core::leases::CreateFurnishedLeaseInput;
use piteo_core::leases::CreateReceiptsInput;
use piteo_core::leases::DeleteLeaseInput;
use piteo_core::leases::SendReceiptsInput;
use piteo_core::leases::UpdateFurnishedLeaseInput;
use piteo_core::owners;
use piteo_core::owners::UpdateIndividualLenderInput;
use piteo_core::properties;
use piteo_core::properties::CreatePropertyInput;
use piteo_core::properties::DeletePropertyInput;
use piteo_core::properties::UpdatePropertyInput;
use piteo_core::tenants;
use piteo_core::tenants::CreateCandidacyInput;
use piteo_core::tenants::CreateTenantInput;
use piteo_core::tenants::DeleteTenantInput;
use piteo_core::tenants::UpdateTenantInput;
use piteo_core::AuthId;
use piteo_core::Candidacy;
use piteo_core::Lease;
use piteo_core::LeaseId;
use piteo_core::Lender;
use piteo_core::Person;
use piteo_core::Property;
use piteo_core::PropertyId;
use piteo_core::Receipt;
use piteo_core::Tenant;
use piteo_core::TenantId;

// # Datasources

pub fn db(db_pool: &DbPool) -> Pg {
    Pg::new(db_pool.clone())
}

// # Auth

pub async fn create_user_with_account(
    db_pool: &DbPool,
    input: CreateUserWithAccountInput,
) -> Result<Person, Error> {
    auth::create_user_with_account(&Pg::new(db_pool.clone()), &Stripe::init(), input).await
}

// # Candidacies

pub fn create_candidacy(db_pool: &DbPool, input: CreateCandidacyInput) -> Result<Candidacy, Error> {
    tenants::create_candidacy(&Pg::new(db_pool.clone()), input)
}

// # Tenants

pub fn create_tenant(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: CreateTenantInput,
) -> Result<Tenant, Error> {
    tenants::create_tenant(&Pg::new(db_pool.clone()), auth_id, input)
}

pub fn update_tenant(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: UpdateTenantInput,
) -> Result<Tenant, Error> {
    tenants::update_tenant(&Pg::new(db_pool.clone()), auth_id, input)
}

pub fn delete_tenant(db_pool: &DbPool, auth_id: &AuthId, id: TenantId) -> Result<TenantId, Error> {
    tenants::delete_tenant(&Pg::new(db_pool.clone()), auth_id, DeleteTenantInput { id })
}

// # Properties

pub fn create_property(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: CreatePropertyInput,
) -> Result<Property, Error> {
    properties::create_property(&Pg::new(db_pool.clone()), auth_id, input)
}

pub fn update_property(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: UpdatePropertyInput,
) -> Result<Property, Error> {
    properties::update_property(&Pg::new(db_pool.clone()), auth_id, input)
}

pub fn delete_property(
    db_pool: &DbPool,
    auth_id: &AuthId,
    id: PropertyId,
) -> Result<TenantId, Error> {
    properties::delete_property(
        &Pg::new(db_pool.clone()),
        auth_id,
        DeletePropertyInput { id },
    )
}

// # Leases

pub fn create_furnished_lease(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: CreateFurnishedLeaseInput,
) -> Result<Lease, Error> {
    leases::create_furnished_lease(&Pg::new(db_pool.clone()), auth_id, input)
}

pub fn update_furnished_lease(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: UpdateFurnishedLeaseInput,
) -> Result<Lease, Error> {
    leases::update_furnished_lease(&Pg::new(db_pool.clone()), auth_id, input)
}

pub fn delete_lease(db_pool: &DbPool, auth_id: &AuthId, id: LeaseId) -> Result<LeaseId, Error> {
    leases::delete_lease(&Pg::new(db_pool.clone()), auth_id, DeleteLeaseInput { id })
}

// # Lenders

pub fn update_individual_lender(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: UpdateIndividualLenderInput,
) -> Result<Lender, Error> {
    owners::update_individual_lender(&Pg::new(db_pool.clone()), auth_id, input)
}

// # Receipts

pub async fn create_receipts(
    db_pool: &DbPool,
    auth_id: &AuthId,
    input: CreateReceiptsInput,
) -> Result<Vec<Receipt>, Error> {
    leases::create_receipts(
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
    leases::send_receipts(
        &Pg::new(db_pool.clone()),
        auth_id,
        &Sendinblue::init(),
        input,
    )
    .await
}
