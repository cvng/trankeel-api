use crate::candidacies::CreateWarrantInput;
use crate::error::Result;
use crate::AuthId;
use crate::Date;
use crate::Tenant;
use async_graphql::InputObject;
use trankeel_core::database::Db;
use trankeel_data::PhoneNumber;
use trankeel_data::TenantData;
use trankeel_data::TenantId;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
#[graphql(name = "TenantUpdateInput")]
pub struct UpdateTenantInput {
    pub birthdate: Option<Date>,
    pub birthplace: Option<String>,
    #[validate(email)]
    pub email: Option<String>, // Email,
    pub id: TenantId,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub note: Option<String>,
    pub phone_number: Option<PhoneNumber>,
    pub is_student: Option<bool>,
    pub warrants: Option<Vec<CreateWarrantInput>>,
}

// # Operation

pub fn update_tenant(db: &impl Db, _auth_id: &AuthId, input: UpdateTenantInput) -> Result<Tenant> {
    input.validate()?;

    db.tenants().update(TenantData {
        id: input.id,
        account_id: Default::default(),
        person_id: Default::default(),
        birthdate: input.birthdate,
        birthplace: input.birthplace,
        email: input.email.map(Into::into),
        first_name: input.first_name,
        last_name: input.last_name,
        note: input.note,
        phone_number: input.phone_number,
        is_student: input.is_student,
        lease_id: None,
        status: None,
    })
}
