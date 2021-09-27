use crate::candidacies::CreateWarrantInput;
use crate::error::Error;
use crate::error::Result;
use crate::AuthId;
use crate::Date;
use crate::Tenant;
use async_graphql::InputObject;
use piteo_core::database::Db;
use piteo_data::AccountId;
use piteo_data::Person;
use piteo_data::PersonId;
use piteo_data::PersonRole;
use piteo_data::PhoneNumber;
use piteo_data::ProfessionalWarrant;
use piteo_data::ProfessionalWarrantId;
use piteo_data::TenantId;
use piteo_data::TenantStatus;
use piteo_data::Warrant;
use piteo_data::WarrantId;
use piteo_data::WarrantIdentity;
use piteo_data::WarrantType;
use piteo_data::WarrantWithIdentity;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
#[graphql(name = "TenantInput")]
pub struct CreateTenantInput {
    pub apl: Option<bool>,
    pub birthdate: Date,
    pub birthplace: Option<String>,
    #[validate(email)]
    pub email: String, // Email,
    pub first_name: String,
    pub last_name: String,
    pub note: Option<String>,
    pub phone_number: Option<PhoneNumber>,
    pub is_student: Option<bool>,
    pub warrants: Option<Vec<CreateWarrantInput>>,
}

// # Operation

pub fn create_tenant(
    db: &impl Db,
    auth_id: &AuthId,
    input: CreateTenantInput,
    account_id: Option<AccountId>,
) -> Result<Tenant> {
    input.validate()?;

    let account_id = match account_id {
        Some(account_id) => account_id,
        None => db.accounts().by_auth_id(auth_id)?.id,
    };

    let tenant = db.tenants().create(Tenant {
        id: TenantId::new_v4(),
        created_at: Default::default(),
        updated_at: Default::default(),
        account_id,
        apl: input.apl,
        birthdate: input.birthdate,
        birthplace: input.birthplace,
        email: input.email.into(),
        first_name: input.first_name,
        last_name: input.last_name,
        note: input.note,
        phone_number: input.phone_number,
        is_student: input.is_student,
        lease_id: None,
        status: TenantStatus::default(),
    })?;

    if let Some(warrant_inputs) = input.warrants {
        add_tenant_warrants(db, &tenant.id, &account_id, warrant_inputs)?;
    }

    Ok(tenant)
}

// # Utils

fn add_tenant_warrants(
    db: &impl Db,
    tenant_id: &TenantId,
    account_id: &AccountId,
    warrant_inputs: Vec<CreateWarrantInput>,
) -> Result<Vec<WarrantWithIdentity>> {
    let mut warrants = vec![];

    for warrant_input in warrant_inputs {
        let warrant = match (
            warrant_input.r#type,
            warrant_input.individual,
            warrant_input.company,
        ) {
            (WarrantType::Person, Some(person_input), _) => db.warrants().create((
                Warrant {
                    id: WarrantId::new_v4(),
                    created_at: Default::default(),
                    updated_at: Default::default(),
                    type_: WarrantType::Person,
                    tenant_id: *tenant_id,
                    individual_id: Default::default(),
                    professional_id: None,
                },
                WarrantIdentity::Individual(Person {
                    id: PersonId::new_v4(),
                    created_at: Default::default(),
                    updated_at: Default::default(),
                    account_id: *account_id,
                    auth_id: None,
                    email: person_input.email,
                    first_name: person_input.first_name,
                    last_name: person_input.last_name,
                    address: Some(person_input.address.into()),
                    phone_number: person_input.phone_number,
                    photo_url: None,
                    role: PersonRole::Warrant,
                }),
            ))?,
            (WarrantType::Visale, _, Some(company_input)) => db.warrants().create((
                Warrant {
                    id: WarrantId::new_v4(),
                    created_at: Default::default(),
                    updated_at: Default::default(),
                    type_: WarrantType::Visale,
                    tenant_id: *tenant_id,
                    individual_id: None,
                    professional_id: Default::default(),
                },
                WarrantIdentity::Professional(ProfessionalWarrant {
                    id: ProfessionalWarrantId::new_v4(),
                    created_at: Default::default(),
                    updated_at: Default::default(),
                    name: company_input.name,
                    identifier: company_input.identifier,
                }),
            ))?,
            (WarrantType::Company, _, Some(company_input)) => db.warrants().create((
                Warrant {
                    id: WarrantId::new_v4(),
                    created_at: Default::default(),
                    updated_at: Default::default(),
                    type_: WarrantType::Company,
                    tenant_id: *tenant_id,
                    individual_id: None,
                    professional_id: Default::default(),
                },
                WarrantIdentity::Professional(ProfessionalWarrant {
                    id: ProfessionalWarrantId::new_v4(),
                    created_at: Default::default(),
                    updated_at: Default::default(),
                    name: company_input.name,
                    identifier: company_input.identifier,
                }),
            ))?,
            _ => return Err(Error::msg("individual or company is missing")),
        };

        warrants.push(warrant);
    }

    Ok(warrants)
}
