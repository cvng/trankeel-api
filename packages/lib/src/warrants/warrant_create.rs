use crate::auth::CreatePersonInput;
use crate::error::Error;
use crate::error::Result;
use async_graphql::InputObject;
use trankeel_data::Account;
use trankeel_data::Candidacy;
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

#[derive(InputObject, Validate)]
pub struct CreateProfessionalWarrantInput {
    pub name: String,
    pub identifier: String,
}

#[derive(InputObject, Validate)]
pub struct CreateWarrantInput {
    pub type_: WarrantType,
    pub individual: Option<CreatePersonInput>,
    pub company: Option<CreateProfessionalWarrantInput>,
}

pub struct CreateWarrantState {
    pub account: Account,
    pub tenant: Option<Tenant>,
    pub candidacy: Option<Candidacy>,
}

pub struct CreateWarrantPayload {
    pub warrant: WarrantWithIdentity,
}

pub(crate) fn create_warrant(
    state: CreateWarrantState,
    input: CreateWarrantInput,
) -> Result<CreateWarrantPayload> {
    let candidacy_id = match state.candidacy {
        Some(candidacy) => Some(candidacy.id),
        None => None,
    };

    let tenant_id = match state.tenant {
        Some(tenant) => Some(tenant.id),
        None => None,
    };

    let warrant = match (input.type_, input.individual, input.company) {
        (WarrantType::Person, Some(person_input), _) => (
            Warrant {
                id: WarrantId::new(),
                created_at: Default::default(),
                updated_at: Default::default(),
                type_: WarrantType::Person,
                tenant_id,
                individual_id: Default::default(),
                professional_id: None,
                candidacy_id,
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
                address: person_input.address.map(Into::into),
                photo_url: None,
                role: PersonRole::Warrant,
                phone_number: person_input.phone_number,
            }),
        ),
        (WarrantType::Visale, _, Some(company_input)) => (
            Warrant {
                id: WarrantId::new(),
                created_at: Default::default(),
                updated_at: Default::default(),
                type_: WarrantType::Visale,
                tenant_id,
                individual_id: None,
                professional_id: Default::default(),
                candidacy_id,
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
                tenant_id,
                individual_id: None,
                professional_id: Default::default(),
                candidacy_id,
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

impl From<WarrantWithIdentity> for CreateWarrantInput {
    fn from(item: WarrantWithIdentity) -> Self {
        match item.1 {
            WarrantIdentity::Individual(person) => Self {
                type_: item.0.type_,
                individual: Some(person.into()),
                company: None,
            },
            WarrantIdentity::Professional(professional) => Self {
                type_: item.0.type_,
                individual: None,
                company: Some(professional.into()),
            },
        }
    }
}

impl From<ProfessionalWarrant> for CreateProfessionalWarrantInput {
    fn from(item: ProfessionalWarrant) -> Self {
        Self {
            name: item.name,
            identifier: item.identifier,
        }
    }
}
