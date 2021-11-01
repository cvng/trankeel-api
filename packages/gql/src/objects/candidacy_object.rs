use super::Discussion;
use super::Person;
use super::Warrant;
use super::Workflow;
use async_graphql::Context;
use async_graphql::Result;
use trankeel::AdvertisementId;
use trankeel::CandidacyId;
use trankeel::CandidacyStatus;
use trankeel::Client;
use trankeel::Date;
use trankeel::DateTime;
use trankeel::PersonId;

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Candidacy {
    pub id: CandidacyId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub status: CandidacyStatus,
    pub advertisement_id: AdvertisementId,
    pub person_id: PersonId,
    pub move_in_date: DateTime,
    pub description: String,
    pub birthdate: Option<Date>,
    pub birthplace: Option<String>,
    pub is_student: Option<bool>,
}

#[async_graphql::ComplexObject]
impl Candidacy {
    async fn candidate(&self, ctx: &Context<'_>) -> Result<Person> {
        Ok(ctx
            .data_unchecked::<Client>()
            .persons()
            .by_id(&self.person_id)?
            .into())
    }

    async fn discussion(&self, ctx: &Context<'_>) -> Result<Discussion> {
        Ok(ctx
            .data_unchecked::<Client>()
            .discussions()
            .by_candidacy_id(&self.id)?
            .into())
    }

    async fn warrants(&self, ctx: &Context<'_>) -> Result<Vec<Warrant>> {
        Ok(ctx
            .data_unchecked::<Client>()
            .warrants()
            .by_candidacy_id(&self.id)?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn workflow(&self, ctx: &Context<'_>) -> Result<Option<Workflow>> {
        Ok(ctx
            .data_unchecked::<Client>()
            .workflows()
            .by_workflowable_id(&self.id)?
            .map(Into::into))
    }
}

impl From<trankeel::Candidacy> for Candidacy {
    fn from(item: trankeel::Candidacy) -> Self {
        Self {
            id: item.id,
            created_at: item.created_at,
            updated_at: item.updated_at,
            status: item.status,
            advertisement_id: item.advertisement_id,
            person_id: item.person_id,
            move_in_date: item.move_in_date,
            description: item.description,
            birthdate: item.birthdate,
            birthplace: item.birthplace,
            is_student: item.is_student,
        }
    }
}
