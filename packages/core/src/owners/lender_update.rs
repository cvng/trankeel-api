use crate::auth::AddressInput;
use crate::companies::UpdateCompanyInput;
use crate::database::Db;
use async_graphql::InputObject;
use eyre::eyre as err;
use eyre::Error;
use piteo_data::AuthId;
use piteo_data::Lender;
use piteo_data::LenderId;
use piteo_data::LenderIdentity;
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
    db: impl Db,
    _auth_id: AuthId,
    input: UpdateIndividualLenderInput,
) -> Result<Lender, Error> {
    input.validate()?;

    let lender = db.lenders().by_id(&input.id)?;

    let person = match &lender {
        LenderIdentity::Individual(_, person) => person,
        _ => return Err(err!("Lender is not an individual")),
    };

    db.persons().update(PersonData {
        id: person.id,
        address: input.individual.address.map(Into::into),
        first_name: input.individual.first_name,
        last_name: input.individual.last_name,
        ..Default::default()
    })?;

    Ok(lender.into())
}
