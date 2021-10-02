use crate::auth::CreatePersonInput;
use crate::error::Result;
use crate::files::CreateFileInput;
use crate::messaging::create_discussion_unauthenticated;
use crate::tenants::create_tenant;
use crate::tenants::CreateTenantInput;
use crate::CreateDiscussionInput;
use async_graphql::InputObject;
use piteo_core::database::Db;
use piteo_core::error::Error;
use piteo_data::Account;
use piteo_data::AdvertisementId;
use piteo_data::AuthId;
use piteo_data::Candidacy;
use piteo_data::CandidacyId;
use piteo_data::CandidacyStatus;
use piteo_data::Date;
use piteo_data::DateTime;
use piteo_data::Discussion;
use piteo_data::PhoneNumber;
use piteo_data::TenantData;
use piteo_data::TenantStatus;
use piteo_data::WarrantType;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
pub struct CreateProfessionalWarrantInput {
    pub name: String,
    pub identifier: String,
}

#[derive(InputObject, Validate)]
pub struct CreateWarrantInput {
    pub r#type: WarrantType,
    pub individual: Option<CreatePersonInput>,
    pub company: Option<CreateProfessionalWarrantInput>,
}

#[derive(InputObject, Validate)]
pub struct CreateCandidacyInput {
    pub advertisement_id: AdvertisementId,
    pub is_student: bool,
    pub first_name: String,
    pub last_name: String,
    pub birthdate: Date,
    #[validate(email)]
    pub email: String, // Email,
    pub phone_number: PhoneNumber,
    pub move_in_date: DateTime,
    pub description: String,
    pub files: Option<Vec<CreateFileInput>>,
    pub warrants: Option<Vec<CreateWarrantInput>>,
}

// # Operation

pub fn create_candidacy(db: &impl Db, input: CreateCandidacyInput) -> Result<Candidacy> {
    input.validate()?;

    let account = db.accounts().by_advertisement_id(&input.advertisement_id)?;
    let no_auth_id = &AuthId::new("".into()); // No auth_id on this endpoint.

    let tenant = create_tenant(
        db,
        no_auth_id,
        CreateTenantInput {
            apl: None,
            birthdate: input.birthdate,
            birthplace: None,
            email: input.email,
            first_name: input.first_name,
            last_name: input.last_name,
            note: None,
            phone_number: Some(input.phone_number),
            is_student: Some(input.is_student),
            warrants: input.warrants,
        },
        Some(account.id),
    )?;

    db.tenants().update(TenantData {
        id: tenant.id,
        status: Some(TenantStatus::Candidate),
        ..Default::default()
    })?;

    let candidacy = db.candidacies().create(Candidacy {
        id: CandidacyId::new_v4(),
        created_at: Default::default(),
        updated_at: Default::default(),
        status: CandidacyStatus::default(),
        advertisement_id: input.advertisement_id,
        tenant_id: tenant.id,
        move_in_date: input.move_in_date,
        description: input.description.clone(),
    })?;

    start_discussion_with_lender(db, &account, &candidacy, input.description)?;

    Ok(candidacy)
}

fn start_discussion_with_lender(
    db: &impl Db,
    account: &Account,
    candidacy: &Candidacy,
    message: String,
) -> Result<Discussion> {
    let initiator = db.persons().by_candidacy_id(&candidacy.id)?;

    // In the context of a candidacy, the recipient is the account owner.
    let recipient = db
        .persons()
        .by_account_id(&account.id)
        .map(|persons| persons.first().cloned())?
        .ok_or_else(|| Error::msg("recipient not found"))?;

    create_discussion_unauthenticated(
        db,
        CreateDiscussionInput {
            initiator_id: initiator.id,
            recipient_id: recipient.id,
            subject_id: Some(candidacy.id),
            message,
        },
    )
}
