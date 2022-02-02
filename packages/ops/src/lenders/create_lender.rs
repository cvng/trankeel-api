use crate::auth::AddressInput;
use crate::error::Error;
use crate::error::Result;
use crate::event::CompanyCreated;
use crate::event::Event;
use crate::event::LenderCreated;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::Account;
use trankeel_data::Company;
use trankeel_data::CompanyId;
use trankeel_data::LegalEntityType;
use trankeel_data::Lender;
use trankeel_data::LenderId;
use trankeel_data::PhoneNumber;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateIndividualLenderInput {
    pub email: String, // Email,
    pub first_name: String,
    pub last_name: String,
    pub address: Option<AddressInput>,
    pub phone_number: Option<PhoneNumber>,
}

#[derive(InputObject, Validate)]
pub struct CreateProfessionalLenderInput {
    pub address: Option<AddressInput>,
    pub email: String, // Email,
    pub legal_entity: String,
    pub legal_entity_identifier: Option<String>,
    pub legal_entity_type: Option<LegalEntityType>,
    pub legal_entity_type_other: Option<String>,
    pub phone_number: Option<PhoneNumber>,
}

#[derive(InputObject, Validate)]
pub struct CreateLenderInput {
    pub individual: Option<CreateIndividualLenderInput>,
    pub company: Option<CreateProfessionalLenderInput>,
}

pub struct CreateLender {
    lender_id: LenderId,
    account: Account,
}

impl CreateLender {
    pub fn new(lender_id: LenderId, account: &Account) -> Self {
        Self {
            lender_id,
            account: account.clone(),
        }
    }
}

impl Command for CreateLender {
    type Input = CreateLenderInput;

    fn run(self, input: Self::Input) -> Result<Vec<Event>> {
        input.validate()?;

        let Self { lender_id, account } = self;

        let company_input = input
            .company
            .ok_or_else(|| Error::msg("cannot create company lender"))?;

        let company = Company {
            id: CompanyId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            address: company_input.address.map(Into::into),
            email: company_input.email.into(),
            legal_entity: company_input.legal_entity,
            legal_entity_identifier: company_input.legal_entity_identifier,
            legal_entity_type: company_input.legal_entity_type,
            legal_entity_type_other: company_input.legal_entity_type_other,
            phone_number: company_input.phone_number,
        };

        let lender = Lender {
            id: lender_id,
            created_at: Default::default(),
            updated_at: Default::default(),
            account_id: account.id,
            individual_id: None,
            company_id: Some(company.id),
        };

        Ok(vec![
            CompanyCreated { company }.into(),
            LenderCreated { lender }.into(),
        ])
    }
}
