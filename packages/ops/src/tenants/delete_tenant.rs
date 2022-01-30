use crate::error::Result;
use crate::event::Event;
use crate::event::TenantDeleted;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::TenantId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct DeleteTenantInput {
    pub id: TenantId,
}

pub struct DeleteTenant;

impl Command for DeleteTenant {
    type Input = DeleteTenantInput;
    type Payload = Vec<Event>;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        Ok(vec![TenantDeleted {
            tenant_id: input.id,
        }
        .into()])
    }
}
