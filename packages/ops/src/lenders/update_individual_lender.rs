use crate::auth::UpdatePersonInput;
use crate::error::Error;
use crate::error::Result;
use crate::event::Event;
use crate::event::LenderUpdated;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::LegalIdentity;
use trankeel_data::LenderId;
use trankeel_data::LenderWithIdentity;
use trankeel_data::Person;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct UpdateIndividualLenderInput {
    pub id: LenderId,
    pub individual: UpdatePersonInput,
}

pub struct UpdateIndividualLender {
    lender: LenderWithIdentity,
}

impl UpdateIndividualLender {
    pub fn new(lender: &LenderWithIdentity) -> Self {
        Self {
            lender: lender.clone(),
        }
    }
}

impl Command for UpdateIndividualLender {
    type Input = UpdateIndividualLenderInput;
    type Payload = Vec<Event>;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self {
            lender: (lender, identity),
        } = self;

        let invididual = match identity {
            LegalIdentity::Individual(invididual) => invididual,
            _ => return Err(Error::msg("lender is not an individual")),
        };

        let invididual = Person {
            address: input.individual.address.map(Into::into),
            first_name: input
                .individual
                .first_name
                .unwrap_or_else(|| invididual.first_name.clone()),
            last_name: input
                .individual
                .last_name
                .unwrap_or_else(|| invididual.last_name.clone()),
            ..invididual
        };

        Ok(vec![LenderUpdated {
            lender,
            identity: invididual.into(),
        }
        .into()])
    }
}
