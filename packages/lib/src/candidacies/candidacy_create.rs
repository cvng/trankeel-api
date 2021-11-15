use crate::auth::CreatePersonInput;
use crate::client::Actor;
use crate::client::Context;
use crate::error::Result;
use crate::files::CreateFileInput;
use crate::messaging;
use crate::messaging::CreateDiscussionState;
use crate::templates::CandidacyCreatedMail;
use crate::warrants::create_warrant;
use crate::warrants::CreateWarrantState;
use crate::CreateDiscussionInput;
use crate::CreateWarrantInput;
use async_graphql::InputObject;
use trankeel_core::activity::trace;
use trankeel_core::activity::Trace;
use trankeel_core::database::Db;
use trankeel_core::error::Error;
use trankeel_core::mailer::Mailer;
use trankeel_data::Account;
use trankeel_data::AdvertisementId;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyId;
use trankeel_data::CandidacyStatus;
use trankeel_data::Date;
use trankeel_data::DateTime;
use trankeel_data::Discussion;
use trankeel_data::DiscussionStatus;
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

// # Operation

pub(crate) async fn create_candidacy(
    ctx: &Context,
    _actor: &Actor,
    input: CreateCandidacyInput,
) -> Result<Candidacy> {
    let db = ctx.db();
    let mailer = ctx.mailer();

    input.validate()?;

    let account = db.accounts().by_advertisement_id(&input.advertisement_id)?;

    let candidate = create_candidate(
        ctx,
        &account,
        CreatePersonInput {
            email: input.email.into(),
            first_name: input.first_name,
            last_name: input.last_name,
            address: None,
            phone_number: Some(input.phone_number),
        },
    )?;

    let candidacy = db.candidacies().create(&Candidacy {
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
    })?;

    if let Some(warrants_input) = input.warrants {
        add_warrants(ctx, &account, &candidacy, warrants_input)?;
    }

    let discussion = start_discussion_with_lender(ctx, &account, &candidate, &candidacy)?;

    db.discussions().update(&Discussion {
        id: discussion.id,
        status: DiscussionStatus::Candidacy,
        ..discussion
    })?;

    trace(db, Trace::CandidacyCreated(candidacy.clone()))?;

    mailer
        .batch(vec![CandidacyCreatedMail::try_new(&candidacy, &candidate)?])
        .await?;

    Ok(candidacy)
}

fn create_candidate(ctx: &Context, account: &Account, input: CreatePersonInput) -> Result<Person> {
    let db = ctx.db();

    let candidate = db.persons().create(&Person {
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

    Ok(candidate)
}

fn add_warrants(
    ctx: &Context,
    account: &Account,
    candidacy: &Candidacy,
    warrants_input: Vec<CreateWarrantInput>,
) -> Result<Vec<WarrantWithIdentity>> {
    let db = ctx.db();

    let mut warrants = vec![];

    for warrant_input in warrants_input {
        let warrant = create_warrant(
            CreateWarrantState {
                account: account.clone(),
                tenant: None,
                candidacy: Some(candidacy.clone()),
            },
            warrant_input,
        )?
        .warrant;
        db.warrants().create(&warrant.clone())?;
        warrants.push(warrant);
    }

    Ok(warrants)
}

fn start_discussion_with_lender(
    ctx: &Context,
    account: &Account,
    candidate: &Person,
    candidacy: &Candidacy,
) -> Result<Discussion> {
    let db = ctx.db();

    // In the context of a candidacy, the recipient is the account owner.
    let recipient = db
        .persons()
        .by_account_id(&account.id)?
        .first()
        .cloned()
        .ok_or_else(|| Error::msg("recipient not found"))?;

    let discussion = messaging::create_discussion(
        CreateDiscussionState {
            account: account.clone(),
        },
        CreateDiscussionInput {
            recipient_id: recipient.id,
            initiator_id: candidate.id,
            message: Some(candidacy.description.clone()),
        },
    )?;

    db.discussions().create(&discussion.discussion)?;
    if let Some(message) = discussion.message.clone() {
        ctx.db().messages().create(&message)?;
    }

    Ok(discussion.discussion)
}
