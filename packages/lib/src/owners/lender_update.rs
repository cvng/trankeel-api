use crate::auth::AddressInput;
use crate::companies::UpdateCompanyInput;
use crate::error::Error;
use crate::error::Result;
use async_graphql::InputObject;
use trankeel_core::database::Db;
use trankeel_data::AuthId;
use trankeel_data::LegalIdentity;
use trankeel_data::Lender;
use trankeel_data::LenderId;
use trankeel_data::PersonData;
use validator::Validate;

// # Input

#[derive(InputObject)]
#[graphql(name = "UserUpdateInput")]
pub struct UpdatePersonInput {
    address: Option<AddressInput>,
    first_name: Option<String>,
    last_name: Option<String>,
}

#[derive(InputObject, Validate)]
#[graphql(name = "LenderIndividualUpdateInput")]
pub struct UpdateIndividualLenderInput {
    id: LenderId,
    individual: UpdatePersonInput,
}

#[derive(InputObject, Validate)]
#[graphql(name = "LenderCompanyUpdateInput")]
pub struct UpdateCompanyLenderUpdateInput {
    company: UpdateCompanyInput,
    id: LenderId,
}

// # Operation

pub fn update_individual_lender(
    db: &impl Db,
    _auth_id: &AuthId,
    input: UpdateIndividualLenderInput,
) -> Result<Lender> {
    input.validate()?;

    let (lender, identity) = db.lenders().by_id(&input.id)?;

    let person = match &identity {
        LegalIdentity::Individual(person) => person,
        _ => return Err(Error::msg("Lender is not an individual")),
    };

    db.persons().update(PersonData {
        id: person.id,
        address: input.individual.address.map(Into::into),
        first_name: input.individual.first_name,
        last_name: input.individual.last_name,
        ..Default::default()
    })?;

    Ok(lender)
}
