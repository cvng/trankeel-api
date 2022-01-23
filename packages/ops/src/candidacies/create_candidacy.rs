use crate::auth::CreatePerson;
use crate::auth::CreatePersonInput;
use crate::error::Result;
use crate::event::Event;
use crate::event::PersonCreated;
use crate::files::CreateFileInput;
use crate::messaging::CreateDiscussion;
use crate::messaging::CreateDiscussionInput;
use crate::messaging::CreateDiscussionPayload;
use crate::warrants::CreateWarrant;
use crate::warrants::CreateWarrantInput;
use crate::warrants::CreateWarrantPayload;
use crate::Command;
use async_graphql::InputObject;
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
use trankeel_data::PersonRole;
use trankeel_data::PhoneNumber;
use trankeel_data::WarrantWithIdentity;
use validator::Validate;

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

pub struct CreateCandidacyPayload {
    pub candidacy: Candidacy,
    pub candidate: Person,
    pub warrants: Option<Vec<WarrantWithIdentity>>,
    pub discussion: Discussion,
    pub messages: Vec<Message>,
}

pub struct CreateCandidacy {
    account: Account,
    account_owner: Person,
}

impl CreateCandidacy {
    pub fn new(account: &Account, account_owner: &Person) -> Self {
        Self {
            account: account.clone(),
            account_owner: account_owner.clone(),
        }
    }
}

impl Command for CreateCandidacy {
    type Input = CreateCandidacyInput;
    type Payload = CreateCandidacyPayload;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self {
            account,
            account_owner,
        } = self;

        let PersonCreated { person: candidate } = CreatePerson::new(&account) //
            .run(CreatePersonInput {
                email: input.email.into(),
                first_name: input.first_name,
                last_name: input.last_name,
                address: None,
                phone_number: Some(input.phone_number),
                role: PersonRole::Candidate,
            })?
            .into_iter()
            .find_map(|event| match event {
                Event::PersonCreated(event) => Some(event),
                _ => None,
            })
            .unwrap();

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
            warrants_input
                .into_iter()
                .map(|input| CreateWarrant::new(&account, None, Some(&candidacy)).run(input))
                .collect::<Result<Vec<_>>>()?
                .into_iter()
                .map(|CreateWarrantPayload { warrant }| Some(warrant))
                .collect()
        } else {
            None
        };

        let CreateDiscussionPayload {
            discussion,
            message,
        } = CreateDiscussion::new(&account).run(CreateDiscussionInput {
            recipient_id: account_owner.id,
            initiator_id: candidate.id,
            message: Some(candidacy.description.clone()),
        })?;

        let messages = vec![message].into_iter().flatten().collect();

        let discussion = Discussion {
            id: discussion.id,
            status: DiscussionStatus::Candidacy,
            ..discussion
        };

        Ok(Self::Payload {
            candidacy,
            candidate,
            warrants,
            discussion,
            messages,
        })
    }
}
