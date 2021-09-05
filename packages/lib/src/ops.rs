use crate::database::Database;
use crate::database::DbPool;
use crate::payment::Stripe;
use piteo_core::auth;
use piteo_core::auth::ops::CreateUserWithAccountInput;
use piteo_core::error::Error;
use piteo_core::properties;
use piteo_core::properties::create_property::CreatePropertyInput;
use piteo_core::properties::delete_property::DeletePropertyInput;
use piteo_core::properties::update_property::UpdatePropertyInput;
use piteo_core::tenants;
use piteo_core::tenants::ops::CreateTenantInput;
use piteo_core::tenants::ops::DeleteTenantInput;
use piteo_core::tenants::ops::UpdateTenantInput;
use piteo_core::AuthId;
use piteo_core::Person;
use piteo_core::Property;
use piteo_core::PropertyId;
use piteo_core::Tenant;
use piteo_core::TenantId;

// # Auth

pub async fn create_user_with_account(
    pool: DbPool,
    input: CreateUserWithAccountInput,
) -> Result<Person, Error> {
    auth::ops::create_user_with_account(Database::new(pool), Stripe::from_env()?, input).await
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
    tenants::ops::create_tenant(Database::new(pool), auth_id, input)
}

pub fn update_tenant(
    pool: DbPool,
    auth_id: AuthId,
    input: UpdateTenantInput,
) -> Result<Tenant, Error> {
    tenants::ops::update_tenant(Database::new(pool), auth_id, input)
}

pub fn delete_tenant(pool: DbPool, auth_id: AuthId, id: TenantId) -> Result<TenantId, Error> {
    tenants::ops::delete_tenant(Database::new(pool), auth_id, DeleteTenantInput { id })
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
    properties::create_property::create_property(Database::new(pool), auth_id, input)
}

pub fn update_property(
    pool: DbPool,
    auth_id: AuthId,
    input: UpdatePropertyInput,
) -> Result<Property, Error> {
    properties::update_property::update_property(Database::new(pool), auth_id, input)
}

pub fn delete_property(pool: DbPool, auth_id: AuthId, id: PropertyId) -> Result<TenantId, Error> {
    properties::delete_property::delete_property(
        Database::new(pool),
        auth_id,
        DeletePropertyInput { id },
    )
}
