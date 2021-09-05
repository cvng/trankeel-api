use crate::database::Db;
use crate::AuthId;
use crate::DateTime;
use crate::Tenant;
use async_graphql::InputObject;
use eyre::Error;
use piteo_data::PhoneNumber;
use piteo_data::TenantData;
use piteo_data::TenantId;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
#[graphql(name = "TenantUpdateInput")]
pub struct UpdateTenantInput {
    pub apl: Option<bool>,
    pub birthdate: Option<DateTime>,
    pub birthplace: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub id: TenantId,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub note: Option<String>,
    pub phone_number: Option<PhoneNumber>,
    pub visale_id: Option<String>,
}

// # Operation

pub fn update_tenant(
    db: impl Db,
    _auth_id: AuthId,
    input: UpdateTenantInput,
) -> Result<Tenant, Error> {
    input.validate()?;

    db.tenants().update(input.into())
}

// # Impls

impl From<UpdateTenantInput> for TenantData {
    fn from(item: UpdateTenantInput) -> Self {
        Self {
            id: item.id,
            account_id: Default::default(),
            apl: item.apl,
            birthdate: item.birthdate,
            birthplace: item.birthplace,
            email: item.email,
            first_name: item.first_name,
            last_name: item.last_name,
            note: item.note,
            phone_number: item.phone_number,
            visale_id: item.visale_id,
        }
    }
}
