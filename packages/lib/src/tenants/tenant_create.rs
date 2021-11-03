use super::CreateWarrantInput;
use crate::error::Error;
use crate::error::Result;
use crate::messaging::discussion_create;
use crate::tenants::create_warrant;
use crate::tenants::CreateWarrantPayload;
use crate::tenants::CreateWarrantState;
use crate::AuthId;
use crate::CreateDiscussionInput;
use crate::Date;
use crate::Tenant;
use async_graphql::InputObject;
use discussion_create::create_discussion;
use discussion_create::CreateDiscussionPayload;
use discussion_create::CreateDiscussionState;
use trankeel_core::activity::trace;
use trankeel_core::activity::Trace;
use trankeel_core::database::Db;
use trankeel_data::Account;
use trankeel_data::DiscussionWithMessages;
use trankeel_data::Person;
use trankeel_data::PersonId;
use trankeel_data::PersonRole;
use trankeel_data::PhoneNumber;
use trankeel_data::TenantId;
use trankeel_data::TenantStatus;
use trankeel_data::TenantWithProfile;
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

pub struct CreateTenantState {
    pub account: Account,
    pub account_owner: Person,
}

pub struct CreateTenantPayload {
    pub tenant: TenantWithProfile,
    pub warrants: Vec<WarrantWithIdentity>,
    pub discussion: DiscussionWithMessages,
}

// # Operation

pub fn run_create_tenant(
    db: &impl Db,
    auth_id: &AuthId,
    input: CreateTenantInput,
    account: Option<Account>,
) -> Result<Tenant> {
    let account = match account {
        Some(account) => account,
        None => db.accounts().by_auth_id(auth_id)?,
    };

    let account_owner = db
        .persons()
        .by_account_id(&account.id)?
        .first()
        .cloned()
        .ok_or_else(|| Error::msg("account owner not found"))?;

    let state = CreateTenantState {
        account,
        account_owner,
    };

    let CreateTenantPayload {
        tenant,
        warrants,
        discussion,
    } = create_tenant(input, state)?;

    trace(
        vec![
            Trace::TenantCreated(tenant.clone()),
            Trace::DiscussionCreated(discussion),
        ]
        .into_iter()
        .chain(
            warrants
                .into_iter()
                .map(Trace::WarrantCreated)
                .collect::<Vec<_>>(),
        )
        .collect(),
    )?;

    Ok(tenant.0)
}

pub fn create_tenant(
    input: CreateTenantInput,
    state: CreateTenantState,
) -> Result<CreateTenantPayload> {
    input.validate()?;

    let profile = Person {
        id: PersonId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        account_id: state.account.id,
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
        account_id: state.account.id,
        person_id: profile.id,
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

    let tenant = (tenant, profile);

    let mut warrants = vec![];

    if let Some(warrants_input) = input.warrants {
        for input in warrants_input {
            let CreateWarrantPayload { warrant } = create_warrant(
                input,
                CreateWarrantState {
                    account: state.account.clone(),
                    tenant: tenant.0.clone(),
                },
            )?;

            warrants.push(warrant)
        }
    }

    let CreateDiscussionPayload { discussion } = create_discussion(
        CreateDiscussionInput {
            initiator_id: profile.id,
            recipient_id: state.account_owner.id,
            message: None,
        },
        CreateDiscussionState {
            account: state.account,
        },
    )?;

    Ok(CreateTenantPayload {
        tenant,
        warrants,
        discussion,
    })
}
