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

    fn run(self, input: Self::Input) -> Result<Vec<Event>> {
        input.validate()?;

        Ok(vec![TenantDeleted {
            tenant_id: input.id,
        }
        .into()])
    }
}
