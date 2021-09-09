use crate::real_database::Database;
use crate::real_database::DbPool;
use crate::real_payment::Stripe;
use piteo_core::auth;
use piteo_core::auth::CreateUserWithAccountInput;
use piteo_core::error::Error;
use piteo_core::leases;
use piteo_core::leases::CreateFurnishedLeaseInput;
use piteo_core::leases::DeleteLeaseInput;
use piteo_core::leases::UpdateFurnishedLeaseInput;
use piteo_core::owners;
use piteo_core::owners::UpdateIndividualLenderInput;
use piteo_core::properties;
use piteo_core::properties::CreatePropertyInput;
use piteo_core::properties::DeletePropertyInput;
use piteo_core::properties::UpdatePropertyInput;
use piteo_core::tenants;
use piteo_core::tenants::CreateTenantInput;
use piteo_core::tenants::DeleteTenantInput;
use piteo_core::tenants::UpdateTenantInput;
use piteo_core::AuthId;
use piteo_core::Lease;
use piteo_core::LeaseId;
use piteo_core::Lender;
use piteo_core::Person;
use piteo_core::Property;
use piteo_core::PropertyId;
use piteo_core::Tenant;
use piteo_core::TenantId;

// # Database

pub fn db(pool: DbPool) -> Database {
    Database::new(pool)
}

// # Auth

pub async fn create_user_with_account(
    pool: DbPool,
    input: CreateUserWithAccountInput,
) -> Result<Person, Error> {
    auth::create_user_with_account(Database::new(pool), Stripe::from_env()?, input).await
}

// # Tenants

pub fn all_tenants(
    pool: DbPool,
    auth_id: AuthId,
    id: Option<TenantId>,
) -> Result<Vec<Tenant>, Error> {
    tenants::all_tenants(Database::new(pool), auth_id, id)
}

pub fn create_tenant(
    pool: DbPool,
    auth_id: AuthId,
    input: CreateTenantInput,
) -> Result<Tenant, Error> {
    tenants::create_tenant(Database::new(pool), auth_id, input)
}

pub fn update_tenant(
    pool: DbPool,
    auth_id: AuthId,
    input: UpdateTenantInput,
) -> Result<Tenant, Error> {
    tenants::update_tenant(Database::new(pool), auth_id, input)
}

pub fn delete_tenant(pool: DbPool, auth_id: AuthId, id: TenantId) -> Result<TenantId, Error> {
    tenants::delete_tenant(Database::new(pool), auth_id, DeleteTenantInput { id })
}

// # Properties

pub fn all_properties(
    pool: DbPool,
    auth_id: AuthId,
    id: Option<PropertyId>,
) -> Result<Vec<Property>, Error> {
    properties::all_properties(Database::new(pool), auth_id, id)
}

pub fn create_property(
    pool: DbPool,
    auth_id: AuthId,
    input: CreatePropertyInput,
) -> Result<Property, Error> {
    properties::create_property(Database::new(pool), auth_id, input)
}

pub fn update_property(
    pool: DbPool,
    auth_id: AuthId,
    input: UpdatePropertyInput,
) -> Result<Property, Error> {
    properties::update_property(Database::new(pool), auth_id, input)
}

pub fn delete_property(pool: DbPool, auth_id: AuthId, id: PropertyId) -> Result<TenantId, Error> {
    properties::delete_property(Database::new(pool), auth_id, DeletePropertyInput { id })
}

// # Leases

pub fn create_furnished_lease(
    pool: DbPool,
    auth_id: AuthId,
    input: CreateFurnishedLeaseInput,
) -> Result<Lease, Error> {
    leases::create_furnished_lease(Database::new(pool), auth_id, input)
}

pub fn update_furnished_lease(
    pool: DbPool,
    auth_id: AuthId,
    input: UpdateFurnishedLeaseInput,
) -> Result<Lease, Error> {
    leases::update_furnished_lease(Database::new(pool), auth_id, input)
}

pub fn delete_lease(pool: DbPool, auth_id: AuthId, id: LeaseId) -> Result<LeaseId, Error> {
    leases::delete_lease(Database::new(pool), auth_id, DeleteLeaseInput { id })
}

// # Lenders

pub fn update_individual_lender(
    pool: DbPool,
    auth_id: AuthId,
    input: UpdateIndividualLenderInput,
) -> Result<Lender, Error> {
    owners::update_individual_lender(Database::new(pool), auth_id, input)
}
