use crate::auth::CreatePersonInput;
use crate::error::Result;
use crate::files::CreateFileInput;
use crate::messaging;
use crate::messaging::CreateDiscussionPayload;
use crate::messaging::CreateDiscussionState;
use crate::tenants::CreateWarrant;
use crate::tenants::CreateWarrantInput;
use crate::CreateDiscussionInput;
use async_graphql::InputObject;
use trankeel_core::context::Context;
use trankeel_core::database::Db;
use trankeel_core::dispatcher::dispatch;
use trankeel_core::dispatcher::AsyncCommand;
use trankeel_core::dispatcher::Command;
use trankeel_core::dispatcher::Event;
use trankeel_core::error::Error;
use trankeel_core::mailer::Mailer;
use trankeel_core::templates::CandidacyCreatedMail;
use trankeel_data::Account;
use trankeel_data::AdvertisementId;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyId;
use trankeel_data::CandidacyStatus;
use trankeel_data::Date;
use trankeel_data::DateTime;
use trankeel_data::Discussion;
use trankeel_data::DiscussionStatus;
use trankeel_data::Message;
use trankeel_data::Person;
use trankeel_data::PersonId;
use trankeel_data::PersonRole;
use trankeel_data::PhoneNumber;
use trankeel_data::WarrantWithIdentity;
use validator::Validate;

// # Input

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
    pub birthdate: Date,
    pub birthplace: Option<String>,
    pub is_student: bool,
    pub files: Option<Vec<CreateFileInput>>,
    pub warrants: Option<Vec<CreateWarrantInput>>,
}

pub struct CreateCandidacyState {
    pub account: Account,
    pub account_owner: Person,
}

pub struct CreateCandidacyPayload {
    pub candidacy: Candidacy,
    pub candidate: Person,
    pub warrants: Option<Vec<WarrantWithIdentity>>,
    pub discussion: Discussion,
    pub messages: Vec<Message>,
}

// # Operation

pub(crate) struct CreateCandidacy<'a> {
    context: &'a Context,
}

impl<'a> CreateCandidacy<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }
}

#[async_trait]
impl<'a> AsyncCommand for CreateCandidacy<'a> {
    type Input = CreateCandidacyInput;
    type Payload = CreateCandidacyPayload;

    async fn run(&self, input: Self::Input) -> Result<Self::Payload> {
        let db = self.context.db();
        let mailer = self.context.mailer();

        let account = db.accounts().by_advertisement_id(&input.advertisement_id)?;
        let account_owner = db
            .persons()
            .by_account_id(&account.id)?
            .first()
            .cloned()
            .ok_or_else(|| Error::msg("recipient not found"))?;

        let state = CreateCandidacyState {
            account,
            account_owner,
        };

        let payload = create_candidacy(state, input)?;

        db.transaction(|| {
            db.persons().create(&payload.candidate)?;
            db.candidacies().create(&payload.candidacy)?;
            if let Some(warrants) = &payload.warrants {
                db.warrants().create_many(warrants)?;
            }
            db.discussions().create(&payload.discussion)?;
            db.messages().create_many(&payload.messages)?;
            dispatch(
                self.context,
                vec![Event::CandidacyCreated(payload.candidacy.clone())],
            )?;
            Ok(())
        })?;

        mailer
            .batch(vec![CandidacyCreatedMail::try_new(
                &payload.candidacy,
                &payload.candidate,
            )?])
            .await?;

        Ok(payload)
    }
}

pub(crate) fn create_candidacy(
    state: CreateCandidacyState,
    input: CreateCandidacyInput,
) -> Result<CreateCandidacyPayload> {
    input.validate()?;

    let CreateCandidacyState {
        account,
        account_owner,
    } = state;

    let candidate = create_candidate(
        &account,
        CreatePersonInput {
            email: input.email.into(),
            first_name: input.first_name,
            last_name: input.last_name,
            address: None,
            phone_number: Some(input.phone_number),
        },
    )?;

    let candidacy = Candidacy {
        id: CandidacyId::new(),
        created_at: Default::default(),
        updated_at: Default::default(),
        status: CandidacyStatus::default(),
        advertisement_id: input.advertisement_id,
        person_id: candidate.id,
        move_in_date: input.move_in_date,
        description: input.description,
        birthdate: Some(input.birthdate),
        birthplace: input.birthplace,
        is_student: Some(input.is_student),
    };

    let warrants = if let Some(warrants_input) = input.warrants {
        Some(add_candidacy_warrants(
            &account,
            &candidacy,
            warrants_input,
        )?)
    } else {
        None
    };

    let CreateDiscussionPayload {
        discussion,
        message,
    } = messaging::create_discussion(
        CreateDiscussionState { account },
        CreateDiscussionInput {
            recipient_id: account_owner.id,
            initiator_id: candidate.id,
            message: Some(candidacy.description.clone()),
        },
    )?;

    let messages = vec![message].into_iter().flatten().collect();

    let discussion = Discussion {
        id: discussion.id,
        status: DiscussionStatus::Candidacy,
        ..discussion
    };

    Ok(CreateCandidacyPayload {
        candidacy,
        candidate,
        warrants,
        discussion,
        messages,
    })
}

fn create_candidate(account: &Account, input: CreatePersonInput) -> Result<Person> {
    let candidate = Person {
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
    };

    Ok(candidate)
}

fn add_candidacy_warrants(
    account: &Account,
    candidacy: &Candidacy,
    warrants_input: Vec<CreateWarrantInput>,
) -> Result<Vec<WarrantWithIdentity>> {
    let mut warrants = vec![];

    for input in warrants_input {
        let warrant = CreateWarrant::new(account, None, Some(candidacy))
            .run(input)?
            .warrant;
        warrants.push(warrant);
    }

    Ok(warrants)
}
