use crate::error::Result;
use crate::messaging;
use crate::messaging::CreateDiscussionPayload;
use crate::messaging::CreateDiscussionState;
use crate::warrants;
use crate::warrants::CreateWarrantState;
use crate::CreateDiscussionInput;
use crate::CreateWarrantInput;
use crate::Date;
use crate::Tenant;
use async_graphql::InputObject;
use trankeel_core::context::Context;
use trankeel_core::database::Db;
use trankeel_core::dispatcher::AsyncCommand;
use trankeel_data::Account;
use trankeel_data::AuthId;
use trankeel_data::Discussion;
use trankeel_data::DiscussionWithMessages;
use trankeel_data::Person;
use trankeel_data::PersonId;
use trankeel_data::PersonRole;
use trankeel_data::PhoneNumber;
use trankeel_data::TenantId;
use trankeel_data::TenantStatus;
use trankeel_data::WarrantWithIdentity;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
pub struct CreateTenantInput {
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

#[derive(Clone)]
pub struct CreateTenantState {
    pub account: Account,
    pub account_owner: Person,
    pub tenant_identity: Option<Person>,
}

#[derive(Clone)]
pub struct CreateTenantPayload {
    pub tenant: Tenant,
    pub tenant_identity: Person,
    pub warrants: Option<Vec<WarrantWithIdentity>>,
    pub discussion: Option<Discussion>,
}

// # Operation

pub(crate) struct CreateTenant<'a> {
    context: &'a Context,
    auth_id: &'a AuthId,
}

impl<'a> CreateTenant<'a> {
    pub fn new(context: &'a Context, auth_id: &'a AuthId) -> Self {
        Self { context, auth_id }
    }
}

#[async_trait]
impl<'a> AsyncCommand for CreateTenant<'a> {
    type Input = CreateTenantInput;
    type Payload = CreateTenantPayload;

    async fn run(&self, input: Self::Input) -> Result<Self::Payload> {
        let db = self.context.db();

        let state = CreateTenantState {
            account: db.accounts().by_auth_id(self.auth_id)?,
            account_owner: db.persons().by_auth_id(self.auth_id)?,
            tenant_identity: None,
        };

        let payload = create_tenant(state, input)?;

        db.transaction(|| {
            db.persons().create(&payload.tenant_identity)?;
            db.tenants().create(&payload.tenant)?;
            if let Some(warrants) = &payload.warrants {
                db.warrants().create_many(warrants)?;
            }
            if let Some(discussion) = &payload.discussion {
                db.discussions().create(discussion)?;
            }
            Ok(())
        })?;

        Ok(payload)
    }
}

pub fn create_tenant(
    state: CreateTenantState,
    input: CreateTenantInput,
) -> Result<CreateTenantPayload> {
    input.validate()?;

    let tenant_identity = state.tenant_identity.clone().unwrap_or(Person {
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
    });

    let tenant = Tenant {
        id: TenantId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        account_id: state.account.id,
        person_id: tenant_identity.id,
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

    let warrants = if let Some(warrants_input) = input.warrants {
        Some(add_tenant_warrants(
            &state.account,
            &tenant,
            warrants_input,
        )?)
    } else {
        None
    };

    let discussion = if state.tenant_identity.is_none() {
        Some(
            start_discussion_with_lender(
                &state.account,
                &state.account_owner,
                &tenant_identity,
                None,
            )?
            .0,
        )
    } else {
        None
    };

    Ok(CreateTenantPayload {
        tenant,
        tenant_identity,
        warrants,
        discussion,
    })
}

// # Utils

pub fn add_tenant_warrants(
    account: &Account,
    tenant: &Tenant,
    warrants_input: Vec<CreateWarrantInput>,
) -> Result<Vec<WarrantWithIdentity>> {
    let mut warrants = vec![];

    for input in warrants_input {
        let warrant = warrants::create_warrant(
            CreateWarrantState {
                account: account.clone(),
                tenant: Some(tenant.clone()),
                candidacy: None,
            },
            input,
        )?
        .warrant;
        warrants.push(warrant);
    }

    Ok(warrants)
}

pub fn start_discussion_with_lender(
    account: &Account,
    account_owner: &Person,
    initiator: &Person,
    message: Option<String>,
) -> Result<DiscussionWithMessages> {
    let CreateDiscussionPayload {
        discussion,
        message,
    } = messaging::create_discussion(
        CreateDiscussionState {
            account: account.clone(),
        },
        CreateDiscussionInput {
            recipient_id: account_owner.id,
            initiator_id: initiator.id,
            message,
        },
    )?;

    let discussion = (discussion, vec![message].into_iter().flatten().collect());

    Ok(discussion)
}
