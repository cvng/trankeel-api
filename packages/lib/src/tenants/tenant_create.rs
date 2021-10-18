use crate::candidacies::CreateWarrantInput;
use crate::error::Error;
use crate::error::Result;
use crate::messaging::discussion_create;
use crate::messaging::CreateDiscussionCommand;
use crate::AuthId;
use crate::Command;
use crate::CreateDiscussionInput;
use crate::Date;
use crate::Tenant;
use async_graphql::InputObject;
use trankeel_core::database::Db;
use trankeel_data::Account;
use trankeel_data::AccountId;
use trankeel_data::DiscussionWithMessages;
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
}

// # Operation

pub fn create_tenant(
    db: &impl Db,
    auth_id: &AuthId,
    input: CreateTenantInput,
    account: Option<Account>,
) -> Result<Tenant> {
    let account = match account {
        Some(account) => account,
        None => db.accounts().by_auth_id(auth_id)?,
    };

    let recipient = db
        .persons()
        .by_account_id(&account.id)?
        .first()
        .cloned()
        .ok_or_else(|| Error::msg("recipient not found"))?;

    let state = State { account, recipient };

    let output = CreateTenantCommand::run(state, input)?;

    let _person = db.persons().create(output.person)?;

    let tenant = db.tenants().create(output.tenant)?;

    let _warrants = output
        .warrants
        .into_iter()
        .map(|warrant| db.warrants().create(warrant))
        .collect::<Result<Vec<_>>>()?;

    let _discussion = db.discussions().create(output.discussion.0)?;

    let _messages = output
        .discussion
        .1
        .into_iter()
        .map(|message| db.messages().create(message))
        .collect::<Result<Vec<_>>>()?;

    Ok(tenant)
}

pub struct State {
    account: Account,
    recipient: Person,
}

pub struct Output {
    tenant: Tenant,
    person: Person,
    warrants: Vec<WarrantWithIdentity>,
    discussion: DiscussionWithMessages,
}

pub(crate) struct CreateTenantCommand;

impl Command for CreateTenantCommand {
    type Input = CreateTenantInput;
    type State = State;
    type Output = Output;

    fn run(state: Self::State, input: Self::Input) -> Result<Self::Output> {
        input.validate()?;

        let account = state.account;
        let recipient = state.recipient;

        let person = Person {
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
        };

        let tenant = Tenant {
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
        };

        let mut warrants = vec![];

        if let Some(warrant_inputs) = input.warrants {
            warrants = add_tenant_warrants(&tenant.id, &account.id, warrant_inputs)?;
        }

        let discussion = start_discussion_with_lender(&account, &recipient, &person)?;

        Ok(Output {
            tenant,
            person,
            warrants,
            discussion,
        })
    }
}

// # Utils

fn add_tenant_warrants(
    tenant_id: &TenantId,
    account_id: &AccountId,
    warrant_inputs: Vec<CreateWarrantInput>,
) -> Result<Vec<WarrantWithIdentity>> {
    let mut warrants = vec![];

    for warrant_input in warrant_inputs {
        let warrant = match (
            warrant_input.type_,
            warrant_input.individual,
            warrant_input.company,
        ) {
            (WarrantType::Person, Some(person_input), _) => (
                Warrant {
                    id: WarrantId::new(),
                    created_at: Default::default(),
                    updated_at: Default::default(),
                    type_: WarrantType::Person,
                    tenant_id: *tenant_id,
                    individual_id: Default::default(),
                    professional_id: None,
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
                    tenant_id: *tenant_id,
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
                    tenant_id: *tenant_id,
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

        warrants.push(warrant);
    }

    Ok(warrants)
}

fn start_discussion_with_lender(
    account: &Account,
    recipient: &Person,
    initiator: &Person,
) -> Result<DiscussionWithMessages> {
    // In the context of a candidacy, the recipient is the account owner.
    let discussion = CreateDiscussionCommand::run(
        discussion_create::State {
            account: account.clone(),
        },
        CreateDiscussionInput {
            initiator_id: initiator.id,
            recipient_id: recipient.id,
            message: None,
        },
    )?
    .discussion;

    Ok(discussion)
}
