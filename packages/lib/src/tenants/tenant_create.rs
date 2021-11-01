use crate::candidacies::CreateWarrantInput;
use crate::error::Error;
use crate::error::Result;
use crate::messaging::create_discussion_unauthenticated;
use crate::AuthId;
use crate::CreateDiscussionInput;
use crate::Date;
use crate::Tenant;
use async_graphql::InputObject;
use trankeel_core::database::Db;
use trankeel_data::Account;
use trankeel_data::AccountId;
use trankeel_data::CandidacyId;
use trankeel_data::Discussion;
use trankeel_data::Person;
use trankeel_data::PersonId;
use trankeel_data::PersonRole;
use trankeel_data::PhoneNumber;
use trankeel_data::ProfessionalWarrant;
use trankeel_data::ProfessionalWarrantId;
use trankeel_data::TenantId;
use trankeel_data::TenantStatus;
use trankeel_data::Warrant;
use trankeel_data::WarrantId;
use trankeel_data::WarrantIdentity;
use trankeel_data::WarrantType;
use trankeel_data::WarrantWithIdentity;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
#[graphql(name = "TenantInput")]
pub struct CreateTenantInput {
    pub apl: Option<bool>,
    pub birthdate: Option<Date>,
    pub birthplace: Option<String>,
    #[validate(email)]
    pub email: String, // Email,
    pub first_name: String,
    pub last_name: String,
    pub note: Option<String>,
    pub phone_number: Option<PhoneNumber>,
    pub is_student: Option<bool>,
    pub warrants: Option<Vec<CreateWarrantInput>>,
    pub person_id: Option<PersonId>,
}

// # Operation

pub fn create_tenant(
    db: &impl Db,
    auth_id: &AuthId,
    input: CreateTenantInput,
    account: Option<Account>,
) -> Result<Tenant> {
    input.validate()?;

    let account = match account {
        Some(account) => account,
        None => db.accounts().by_auth_id(auth_id)?,
    };

    let person = if let Some(person_id) = input.person_id {
        db.persons().by_id(&person_id)?
    } else {
        let person = db.persons().create(Person {
            id: PersonId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            account_id: account.id,
            auth_id: None, // Not authenticable when created.
            email: input.email.clone().into(),
            first_name: input.first_name.clone(),
            last_name: input.last_name.clone(),
            address: None,
            photo_url: None,
            role: PersonRole::Tenant,
            phone_number: input.phone_number.clone(),
        })?;

        start_discussion_with_lender(db, &account, &person)?;

        person
    };

    let tenant = db.tenants().create(Tenant {
        id: TenantId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        account_id: account.id,
        person_id: person.id,
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
        add_warrants(db, &account.id, Some(tenant.id), None, warrant_inputs)?;
    }

    Ok(tenant)
}

// # Utils

pub fn add_warrants(
    db: &impl Db,
    account_id: &AccountId,
    tenant_id: Option<TenantId>,
    candidacy_id: Option<CandidacyId>,
    warrant_inputs: Vec<CreateWarrantInput>,
) -> Result<Vec<WarrantWithIdentity>> {
    let mut warrants = vec![];

    for warrant_input in warrant_inputs {
        let warrant = match (
            warrant_input.type_,
            warrant_input.individual,
            warrant_input.company,
        ) {
            (WarrantType::Person, Some(person_input), _) => db.warrants().create((
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
                    account_id: *account_id,
                    auth_id: None,
                    email: person_input.email,
                    first_name: person_input.first_name,
                    last_name: person_input.last_name,
                    address: person_input.address.map(Into::into),
                    photo_url: None,
                    role: PersonRole::Warrant,
                    phone_number: person_input.phone_number,
                }),
            ))?,
            (WarrantType::Visale, _, Some(company_input)) => db.warrants().create((
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
            ))?,
            (WarrantType::Company, _, Some(company_input)) => db.warrants().create((
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
            ))?,
            _ => return Err(Error::msg("individual or company is missing")),
        };

        warrants.push(warrant);
    }

    Ok(warrants)
}

pub fn start_discussion_with_lender(
    db: &impl Db,
    account: &Account,
    initiator: &Person,
) -> Result<Discussion> {
    // In the context of a candidacy, the recipient is the account owner.
    let recipient = db
        .persons()
        .by_account_id(&account.id)?
        .first()
        .cloned()
        .ok_or_else(|| Error::msg("recipient not found"))?;

    create_discussion_unauthenticated(
        db,
        CreateDiscussionInput {
            initiator_id: initiator.id,
            recipient_id: recipient.id,
            message: None,
        },
    )
}
