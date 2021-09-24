use crate::auth::CreatePersonInput;
use crate::database::Db;
use crate::files::CreateFileInput;
use async_graphql::InputObject;
use eyre::Error;
use piteo_data::Candidacy;
use piteo_data::Date;
use piteo_data::Email;
use piteo_data::PhoneNumber;
use piteo_data::WarrantType;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
pub struct CreateWarrantInput {
    r#type: WarrantType,
    person: Option<CreatePersonInput>,
    identifier: Option<String>,
}

#[derive(InputObject, Validate)]
pub struct CreateCandidacyInput {
    is_student: bool,
    first_name: String,
    last_name: String,
    birthdate: Date,
    email: Email,
    phone_number: PhoneNumber,
    move_in_date: Date,
    description: String,
    files: Option<Vec<CreateFileInput>>,
    warrants: Option<Vec<CreateWarrantInput>>,
}

// # Operation

pub fn create_candidacy(_db: &impl Db, input: CreateCandidacyInput) -> Result<Candidacy, Error> {
    input.validate()?;
    todo!()
}
