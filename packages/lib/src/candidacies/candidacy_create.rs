use crate::auth::CreatePersonInput;
use crate::error::Result;
use crate::files::CreateFileInput;
use crate::messaging::push_message;
use crate::templates::CandidacyCreatedMail;
use crate::tenants::add_warrants;
use crate::tenants::start_discussion_with_lender;
use crate::PushMessageInput;
use async_graphql::InputObject;
use trankeel_core::activity::trace;
use trankeel_core::activity::Trace;
use trankeel_core::database::Db;
use trankeel_core::mailer::Mailer;
use trankeel_data::Account;
use trankeel_data::AdvertisementId;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyId;
use trankeel_data::CandidacyStatus;
use trankeel_data::Date;
use trankeel_data::DateTime;
use trankeel_data::DiscussionData;
use trankeel_data::DiscussionStatus;
use trankeel_data::Person;
use trankeel_data::PersonId;
use trankeel_data::PersonRole;
use trankeel_data::PhoneNumber;
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
    #[validate(email)]
    pub email: String, // Email,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: PhoneNumber,
    pub move_in_date: DateTime,
    pub description: String,
    pub apl: Option<bool>,
    pub birthdate: Date,
    pub birthplace: Option<String>,
    pub is_student: bool,
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

    let candidate = create_candidate(
        db,
        &account,
        CreatePersonInput {
            email: input.email.into(),
            first_name: input.first_name,
            last_name: input.last_name,
            address: None,
            phone_number: Some(input.phone_number),
        },
    )?;

    let candidacy = db.candidacies().create(Candidacy {
        id: CandidacyId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        status: CandidacyStatus::default(),
        advertisement_id: input.advertisement_id,
        person_id: candidate.id,
        move_in_date: input.move_in_date,
        description: input.description,
        apl: input.apl,
        birthdate: Some(input.birthdate),
        birthplace: input.birthplace,
        is_student: Some(input.is_student),
    })?;

    if let Some(warrant_inputs) = input.warrants {
        add_warrants(db, &account.id, None, Some(candidacy.id), warrant_inputs)?;
    }

    trace(db, Trace::CandidacyCreated(candidacy.clone()))?;

    let discussion = db.discussions().by_initiator_id(&candidate.id)?;

    let discussion = db.discussions().update(DiscussionData {
        id: discussion.id,
        status: Some(DiscussionStatus::Candidacy),
        ..Default::default()
    })?;

    push_message(
        db,
        PushMessageInput {
            discussion_id: discussion.id,
            sender_id: candidate.id,
            message: candidacy.description.clone(),
        },
    )?;

    mailer
        .batch(vec![CandidacyCreatedMail::try_new(&candidacy, &candidate)?])
        .await?;

    Ok(candidacy)
}

fn create_candidate(db: &impl Db, account: &Account, input: CreatePersonInput) -> Result<Person> {
    let candidate = db.persons().create(Person {
        id: PersonId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        account_id: account.id,
        auth_id: None,
        email: input.email,
        first_name: input.first_name,
        last_name: input.last_name,
        address: None,
        photo_url: None,
        role: PersonRole::Candidate,
        phone_number: input.phone_number,
    })?;

    start_discussion_with_lender(db, account, &candidate)?;

    Ok(candidate)
}
