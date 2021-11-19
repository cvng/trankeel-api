use crate::error::Result;
use crate::warrants::CreateWarrantInput;
use crate::Date;
use crate::Tenant;
use async_graphql::InputObject;
use trankeel_data::PhoneNumber;
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

pub struct UpdateTenantState {
    pub tenant: Tenant,
}

pub struct UpdateTenantPayload {
    pub tenant: Tenant,
}

// # Operation

pub fn update_tenant(
    state: UpdateTenantState,
    input: UpdateTenantInput,
) -> Result<UpdateTenantPayload> {
    input.validate()?;

    let tenant = state.tenant;

    let tenant = Tenant {
        id: input.id,
        birthdate: input.birthdate,
        birthplace: input.birthplace,
        email: input.email.map(Into::into).unwrap_or(tenant.email),
        first_name: input.first_name.unwrap_or(tenant.first_name),
        last_name: input.last_name.unwrap_or(tenant.last_name),
        note: input.note,
        phone_number: input.phone_number,
        is_student: input.is_student,
        ..tenant
    };

    Ok(UpdateTenantPayload { tenant })
}
