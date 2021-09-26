use super::create_tenant;
use super::CreateTenantInput;
use crate::auth::CreatePersonInput;
use crate::files::CreateFileInput;
use async_graphql::InputObject;
use piteo_core::database::Db;
use piteo_core::error::Error;
use piteo_data::AdvertisementId;
use piteo_data::AuthId;
use piteo_data::Candidacy;
use piteo_data::CandidacyId;
use piteo_data::CandidacyStatus;
use piteo_data::Date;
use piteo_data::DateTime;
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
    advertisement_id: AdvertisementId,
    is_student: bool,
    first_name: String,
    last_name: String,
    birthdate: Date,
    #[validate(email)]
    email: String, // Email,
    phone_number: PhoneNumber,
    move_in_date: DateTime,
    description: String,
    files: Option<Vec<CreateFileInput>>,
    warrants: Option<Vec<CreateWarrantInput>>,
}

// # Operation

pub fn create_candidacy(db: &impl Db, input: CreateCandidacyInput) -> Result<Candidacy, Error> {
    input.validate()?;

    let account = db.accounts().by_advertisement_id(&input.advertisement_id)?;
    let auth_id = &AuthId::new("".into()); // No auth_id on this endpoint.

    let tenant = create_tenant(
        db,
        auth_id,
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

    db.candidacies().create(Candidacy {
        id: CandidacyId::new_v4(),
        created_at: Default::default(),
        updated_at: Default::default(),
        status: CandidacyStatus::default(),
        advertisement_id: input.advertisement_id,
        tenant_id: tenant.id,
        move_in_date: input.move_in_date,
        description: input.description,
    })
}
