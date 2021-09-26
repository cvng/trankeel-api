use crate::auth::AddressInput;
use crate::companies::UpdateCompanyInput;
use async_graphql::InputObject;
use piteo_core::database::Db;
use piteo_core::error::Error;
use piteo_data::AuthId;
use piteo_data::LegalIdentity;
use piteo_data::Lender;
use piteo_data::LenderId;
use piteo_data::PersonData;
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
) -> Result<Lender, Error> {
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
