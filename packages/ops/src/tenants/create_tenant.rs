use crate::command::Command;
use crate::error::Result;
use crate::event::Event;
use crate::event::TenantCreated;
use crate::messaging::CreateDiscussion;
use crate::messaging::CreateDiscussionInput;
use crate::warrants::CreateWarrant;
use crate::warrants::CreateWarrantInput;
use async_graphql::InputObject;
use trankeel_data::Account;
use trankeel_data::Date;
use trankeel_data::Person;
use trankeel_data::PersonId;
use trankeel_data::PersonRole;
use trankeel_data::PhoneNumber;
use trankeel_data::Tenant;
use trankeel_data::TenantId;
use validator::Validate;

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

pub struct CreateTenant {
    tenant_id: TenantId,
    account: Account,
    account_owner: Person,
    identity: Option<Person>,
}

impl CreateTenant {
    pub fn new(
        tenant_id: TenantId,
        account: &Account,
        account_owner: &Person,
        identity: Option<&Person>,
    ) -> Self {
        Self {
            tenant_id,
            account: account.clone(),
            account_owner: account_owner.clone(),
            identity: identity.cloned(),
        }
    }
}

impl Command for CreateTenant {
    type Input = CreateTenantInput;

    fn run(self, input: Self::Input) -> Result<Vec<Event>> {
        input.validate()?;

        let Self {
            tenant_id,
            account,
            account_owner,
            identity,
        } = self;

        let identity_already_exists = identity.is_some();

        // Create tenant identity (used for messaging).
        let identity = identity.unwrap_or(Person {
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
        });

        // Create tenant profile.
        let tenant = Tenant {
            id: tenant_id,
            created_at: Default::default(),
            updated_at: Default::default(),
            account_id: account.id,
            person_id: identity.id,
            birthdate: input.birthdate,
            birthplace: input.birthplace,
            email: input.email.into(),
            first_name: input.first_name,
            last_name: input.last_name,
            note: input.note,
            phone_number: input.phone_number,
            is_student: input.is_student,
            lease_id: None,
        };

        // Affect warrants if provided.
        let warrants = if let Some(warrants_input) = input.warrants {
            warrants_input
                .into_iter()
                .map(|input| CreateWarrant::new(&account, Some(&tenant), None).run(input))
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .map(|events| {
                    events.into_iter().find_map(|event| match event {
                        Event::WarrantCreated(event) => Some(event.warrant),
                        _ => None,
                    })
                })
                .collect()
        } else {
            None
        };

        // Create discussion if needed.
        let discussion = if !identity_already_exists {
            CreateDiscussion::new(&account)
                .run(CreateDiscussionInput {
                    recipient_id: account_owner.id,
                    initiator_id: identity.id,
                    message: None,
                })?
                .into_iter()
                .find_map(|event| match event {
                    Event::DiscussionCreated(event) => Some(event.discussion),
                    _ => None,
                })
        } else {
            None
        };

        Ok(vec![TenantCreated {
            tenant,
            identity: Some(identity),
            warrants,
            discussion,
        }
        .into()])
    }
}
