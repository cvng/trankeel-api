use crate::auth::CreatePersonInput;
use crate::error::Result;
use crate::files::CreateFileInput;
use crate::messaging::push_message;
use crate::templates::CandidacyCreatedMail;
use crate::tenants::create_tenant;
use crate::tenants::CreateTenantInput;
use crate::PushMessageInput;
use async_graphql::InputObject;
use trankeel_core::activity::trace;
use trankeel_core::activity::Trace;
use trankeel_core::database::Db;
use trankeel_core::mailer::Mailer;
use trankeel_data::AdvertisementId;
use trankeel_data::AuthId;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyId;
use trankeel_data::CandidacyStatus;
use trankeel_data::Date;
use trankeel_data::DateTime;
use trankeel_data::DiscussionData;
use trankeel_data::DiscussionStatus;
use trankeel_data::PhoneNumber;
use trankeel_data::TenantData;
use trankeel_data::TenantStatus;
use trankeel_data::WarrantType;
use validator::Validate;

// # Input

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

pub async fn create_candidacy(
    db: &impl Db,
    mailer: &impl Mailer,
    input: CreateCandidacyInput,
) -> Result<Candidacy> {
    input.validate()?;

    let account = db.accounts().by_advertisement_id(&input.advertisement_id)?;
    let no_auth_id = AuthId::new("".into()); // No auth_id on this endpoint.

    let tenant = create_tenant(
        db,
        &no_auth_id,
        CreateTenantInput {
            apl: None,
            birthdate: Some(input.birthdate),
            birthplace: None,
            email: input.email,
            first_name: input.first_name,
            last_name: input.last_name,
            note: None,
            phone_number: Some(input.phone_number),
            is_student: Some(input.is_student),
            warrants: input.warrants,
        },
        Some(account),
    )?;

    db.tenants().update(TenantData {
        id: tenant.id,
        status: Some(TenantStatus::Candidate),
        ..Default::default()
    })?;

    let candidacy = db.candidacies().create(Candidacy {
        id: CandidacyId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        status: CandidacyStatus::default(),
        advertisement_id: input.advertisement_id,
        tenant_id: tenant.id,
        move_in_date: input.move_in_date,
        description: input.description,
    })?;

    trace(db, Trace::CandidacyCreated(candidacy.clone()))?;

    let discussion = db.discussions().by_initiator_id(&tenant.person_id)?;

    let discussion = db.discussions().update(DiscussionData {
        id: discussion.id,
        status: Some(DiscussionStatus::Candidacy),
        ..Default::default()
    })?;

    push_message(
        db,
        PushMessageInput {
            discussion_id: discussion.id,
            sender_id: tenant.person_id,
            message: candidacy.description.clone(),
        },
    )?;

    let candidate = db.persons().by_id(&tenant.person_id)?;

    mailer
        .batch(vec![CandidacyCreatedMail::try_new(&candidacy, &candidate)?])
        .await?;

    Ok(candidacy)
}
