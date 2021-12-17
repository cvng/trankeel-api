use crate::error::Result;
use crate::Command;
use trankeel_data::TenantId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct DeleteTenantInput {
    pub id: TenantId,
}

pub struct DeleteTenant;

impl Command for DeleteTenant {
    type Input = DeleteTenantInput;
    type Payload = TenantId;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        Ok(input.id)
    }
}
