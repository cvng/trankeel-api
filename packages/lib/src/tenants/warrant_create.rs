use crate::auth::CreatePersonInput;
use crate::candidacies::CreateProfessionalWarrantInput;
use crate::error::Result;
use trankeel_core::error::Error;
use trankeel_data::Account;
use trankeel_data::Person;
use trankeel_data::PersonId;
use trankeel_data::PersonRole;
use trankeel_data::ProfessionalWarrant;
use trankeel_data::ProfessionalWarrantId;
use trankeel_data::Tenant;
use trankeel_data::Warrant;
use trankeel_data::WarrantId;
use trankeel_data::WarrantIdentity;
use trankeel_data::WarrantType;
use trankeel_data::WarrantWithIdentity;
use validator::Validate;

#[derive(Clone, InputObject, Validate)]
pub struct CreateWarrantInput {
    pub type_: WarrantType,
    pub individual: Option<CreatePersonInput>,
    pub company: Option<CreateProfessionalWarrantInput>,
}

pub struct CreateWarrantState {
    pub account: Account,
    pub tenant: Tenant,
}

pub struct CreateWarrantPayload {
    pub warrant: WarrantWithIdentity,
}

pub fn create_warrant(
    input: CreateWarrantInput,
    state: CreateWarrantState,
) -> Result<CreateWarrantPayload> {
    input.validate()?;

    let warrant = match (input.type_, input.individual, input.company) {
        (WarrantType::Person, Some(person_input), _) => (
            Warrant {
                id: WarrantId::new(),
                created_at: Default::default(),
                updated_at: Default::default(),
                type_: WarrantType::Person,
                tenant_id: state.tenant.id,
                individual_id: Default::default(),
                professional_id: None,
            },
            WarrantIdentity::Individual(Person {
                id: PersonId::new(),
                created_at: Default::default(),
                updated_at: Default::default(),
                account_id: state.account.id,
                auth_id: None,
                email: person_input.email,
                first_name: person_input.first_name,
                last_name: person_input.last_name,
                address: Some(person_input.address.into()),
                phone_number: person_input.phone_number,
                photo_url: None,
                role: PersonRole::Warrant,
            }),
        ),
        (WarrantType::Visale, _, Some(company_input)) => (
            Warrant {
                id: WarrantId::new(),
                created_at: Default::default(),
                updated_at: Default::default(),
                type_: WarrantType::Visale,
                tenant_id: state.tenant.id,
                individual_id: None,
                professional_id: Default::default(),
            },
            WarrantIdentity::Professional(ProfessionalWarrant {
                id: ProfessionalWarrantId::new(),
                created_at: Default::default(),
                updated_at: Default::default(),
                name: company_input.name,
                identifier: company_input.identifier,
            }),
        ),
        (WarrantType::Company, _, Some(company_input)) => (
            Warrant {
                id: WarrantId::new(),
                created_at: Default::default(),
                updated_at: Default::default(),
                type_: WarrantType::Company,
                tenant_id: state.tenant.id,
                individual_id: None,
                professional_id: Default::default(),
            },
            WarrantIdentity::Professional(ProfessionalWarrant {
                id: ProfessionalWarrantId::new(),
                created_at: Default::default(),
                updated_at: Default::default(),
                name: company_input.name,
                identifier: company_input.identifier,
            }),
        ),
        _ => return Err(Error::msg("individual or company is missing")),
    };

    Ok(CreateWarrantPayload { warrant })
}
