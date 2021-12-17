use crate::error::Result;
use crate::warrants::CreateWarrantInput;
use async_graphql::InputObject;
use trankeel_core::dispatcher::Command;
use trankeel_data::Date;
use trankeel_data::PhoneNumber;
use trankeel_data::Tenant;
use trankeel_data::TenantId;
use validator::Validate;

#[derive(InputObject, Validate)]
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
    pub warrants: Option<Vec<CreateWarrantInput>>, // TODO
}

pub struct UpdateTenantPayload {
    pub tenant: Tenant,
}

pub struct UpdateTenant {
    tenant: Tenant,
}

impl UpdateTenant {
    pub fn new(tenant: &Tenant) -> Self {
        Self {
            tenant: tenant.clone(),
        }
    }
}

impl Command for UpdateTenant {
    type Input = UpdateTenantInput;
    type Payload = UpdateTenantPayload;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self { tenant } = self;

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

        Ok(Self::Payload { tenant })
    }
}
