use crate::auth::CreatePerson;
use crate::auth::CreatePersonInput;
use crate::error::Result;
use crate::event::CandidacyCreated;
use crate::event::DiscussionCreated;
use crate::event::Event;
use crate::event::MessagePushed;
use crate::event::PersonCreated;
use crate::event::WarrantCreated;
use crate::files::CreateFileInput;
use crate::messaging::CreateDiscussion;
use crate::messaging::CreateDiscussionInput;
use crate::warrants::CreateWarrant;
use crate::warrants::CreateWarrantInput;
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
use trankeel_data::Person;
use trankeel_data::PersonRole;
use trankeel_data::PhoneNumber;
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

pub struct CreateCandidacy {
    candidacy_id: CandidacyId,
    account: Account,
    account_owner: Person,
}

impl CreateCandidacy {
    pub fn new(candidacy_id: CandidacyId, account: &Account, account_owner: &Person) -> Self {
        Self {
            candidacy_id,
            account: account.clone(),
            account_owner: account_owner.clone(),
        }
    }
}

impl Command for CreateCandidacy {
    type Input = CreateCandidacyInput;
    type Payload = Vec<Event>;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self {
            candidacy_id,
            account,
            account_owner,
        } = self;

        let candidate = CreatePerson::new(&account)
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
                Event::PersonCreated(event) => Some(event.person),
                _ => None,
            })
            .unwrap();

        let candidacy = Candidacy {
            id: candidacy_id,
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
                .map(|events| {
                    events.into_iter().find_map(|event| match event {
                        Event::WarrantCreated(event) => Some(event.warrant),
                        _ => None,
                    })
                })
                .collect::<Option<Vec<_>>>()
        } else {
            None
        };

        let (discussion, message) = CreateDiscussion::new(&account)
            .run(CreateDiscussionInput {
                recipient_id: account_owner.id,
                initiator_id: candidate.id,
                message: Some(candidacy.description.clone()),
            })?
            .into_iter()
            .find_map(|event| match event {
                Event::DiscussionCreated(event) => Some((event.discussion, event.message)),
                _ => None,
            })
            .unwrap();

        let messages = vec![message].into_iter().flatten();

        let discussion = Discussion {
            id: discussion.id,
            status: DiscussionStatus::Candidacy,
            ..discussion
        };

        Ok(vec![
            PersonCreated { person: candidate }.into(),
            CandidacyCreated { candidacy }.into(),
            DiscussionCreated {
                discussion,
                message: None,
            }
            .into(),
        ]
        .into_iter()
        .chain(
            warrants
                .map(|warrants| {
                    warrants
                        .into_iter()
                        .map(|warrant| WarrantCreated { warrant }.into())
                        .collect()
                })
                .unwrap_or_else(Vec::new),
        )
        .chain(
            messages
                .into_iter()
                .map(|message| MessagePushed { message }.into())
                .collect::<Vec<_>>(),
        )
        .collect())
    }
}
