use crate::database::Db;
use crate::AuthId;
use crate::Date;
use crate::Tenant;
use async_graphql::InputObject;
use eyre::Error;
use piteo_data::PhoneNumber;
use piteo_data::TenantId;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
#[graphql(name = "TenantInput")]
pub struct CreateTenantInput {
    pub apl: Option<bool>,
    pub birthdate: Date,
    pub birthplace: Option<String>,
    #[validate(email)]
    pub email: String, // Email,
    pub first_name: String,
    pub last_name: String,
    pub note: Option<String>,
    pub phone_number: Option<PhoneNumber>,
    pub visale_id: Option<String>,
}

// # Operation

pub fn create_tenant(
    db: &impl Db,
    auth_id: &AuthId,
    input: CreateTenantInput,
) -> Result<Tenant, Error> {
    input.validate()?;

    let account = db.accounts().by_auth_id(auth_id)?;

    db.tenants().create(Tenant {
        id: TenantId::new_v4(),
        created_at: Default::default(),
        updated_at: Default::default(),
        account_id: account.id,
        apl: input.apl.unwrap_or_default(),
        birthdate: input.birthdate,
        birthplace: input.birthplace,
        email: input.email.into(),
        first_name: input.first_name,
        last_name: input.last_name,
        note: input.note,
        phone_number: input.phone_number,
        visale_id: input.visale_id,
        lease_id: None,
    })
}
