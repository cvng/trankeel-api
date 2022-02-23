use async_graphql::SimpleObject;
use trankeel::DateTime;
use trankeel::Requirement;
use trankeel::StepId;

#[derive(SimpleObject)]
pub struct Step {
    pub id: StepId,
    pub label: String,
    pub requirements: Option<Vec<Requirement>>,
    pub completed: bool,
    pub completed_at: Option<DateTime>,
}

impl From<trankeel::Step> for Step {
    fn from(item: trankeel::Step) -> Self {
        Self {
            id: item.id,
            label: item.label,
            completed: item.completed,
            completed_at: item.updated_at,
            requirements: item
                .requirements
                .map(|requirements| requirements.requirements),
        }
    }
}
