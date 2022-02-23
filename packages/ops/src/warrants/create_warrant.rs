use crate::auth::AddressInput;
use crate::command::Command;
use crate::error::Error;
use crate::error::Result;
use crate::event::Event;
use crate::event::WarrantCreated;
use async_graphql::InputObject;
use trankeel_data::Account;
use trankeel_data::Candidacy;
use trankeel_data::Person;
use trankeel_data::PersonId;
use trankeel_data::PersonRole;
use trankeel_data::PhoneNumber;
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
pub struct CreateIndividualWarrantInput {
    pub email: String, // Email,
    pub first_name: String,
    pub last_name: String,
    pub address: Option<AddressInput>,
    pub phone_number: Option<PhoneNumber>,
}

#[derive(InputObject, Validate)]
pub struct CreateProfessionalWarrantInput {
    pub name: String,
    pub identifier: String,
}

#[derive(InputObject, Validate)]
pub struct CreateWarrantInput {
    pub type_: WarrantType,
    pub individual: Option<CreateIndividualWarrantInput>,
    pub company: Option<CreateProfessionalWarrantInput>,
}

pub struct CreateWarrant {
    account: Account,
    tenant: Option<Tenant>,
    candidacy: Option<Candidacy>,
}

impl CreateWarrant {
    pub fn new(account: &Account, tenant: Option<&Tenant>, candidacy: Option<&Candidacy>) -> Self {
        Self {
            account: account.clone(),
            tenant: tenant.cloned(),
            candidacy: candidacy.cloned(),
        }
    }
}

impl Command for CreateWarrant {
    type Input = CreateWarrantInput;

    fn run(self, input: Self::Input) -> Result<Vec<Event>> {
        input.validate()?;

        let Self {
            account,
            tenant,
            candidacy,
        } = self;

        let candidacy_id = match candidacy {
            Some(candidacy) => Some(candidacy.id),
            None => None,
        };

        let tenant_id = match tenant {
            Some(tenant) => Some(tenant.id),
            None => None,
        };

        let (warrant, identity) = match (input.type_, input.individual, input.company) {
            (WarrantType::Person, Some(individual_input), _) => (
                Warrant {
                    id: WarrantId::new(),
                    created_at: Default::default(),
                    updated_at: Default::default(),
                    type_: WarrantType::Person,
                    tenant_id,
                    candidacy_id,
                    individual_id: None,
                    professional_id: None,
                },
                WarrantIdentity::Individual(Person {
                    id: PersonId::new(),
                    created_at: Default::default(),
                    updated_at: Default::default(),
                    account_id: account.id,
                    auth_id: None,
                    email: individual_input.email.into(),
                    first_name: individual_input.first_name,
                    last_name: individual_input.last_name,
                    address: individual_input.address.map(Into::into),
                    photo_url: None,
                    role: PersonRole::Warrant,
                    phone_number: individual_input.phone_number,
                }),
            ),
            (WarrantType::Visale, _, Some(company_input)) => (
                Warrant {
                    id: WarrantId::new(),
                    created_at: Default::default(),
                    updated_at: Default::default(),
                    type_: WarrantType::Visale,
                    tenant_id,
                    candidacy_id,
                    individual_id: None,
                    professional_id: None,
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
                    candidacy_id,
                    individual_id: None,
                    professional_id: None,
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

        Ok(vec![WarrantCreated {
            warrant: WarrantWithIdentity { warrant, identity },
        }
        .into()])
    }
}

impl From<WarrantWithIdentity> for CreateWarrantInput {
    fn from(item: WarrantWithIdentity) -> Self {
        match item.identity {
            WarrantIdentity::Individual(person) => Self {
                type_: item.warrant.type_,
                individual: Some(person.into()),
                company: None,
            },
            WarrantIdentity::Professional(professional) => Self {
                type_: item.warrant.type_,
                individual: None,
                company: Some(professional.into()),
            },
        }
    }
}

impl From<Person> for CreateIndividualWarrantInput {
    fn from(item: Person) -> Self {
        Self {
            email: item.email.inner().to_string(),
            first_name: item.first_name,
            last_name: item.last_name,
            address: item.address.map(Into::into),
            phone_number: item.phone_number,
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
