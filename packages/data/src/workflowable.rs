use crate::id;
use crate::Candidacy;
use crate::CandidacyId;
use crate::Workflow;
use async_graphql::Union;
use fake::Fake;
use serde::Serialize;

id!(WorkflowableId);

impl From<CandidacyId> for WorkflowableId {
    fn from(item: CandidacyId) -> Self {
        Self(item.0)
    }
}

pub type WorkflowableRow = (Workflow, Option<Candidacy>);

#[derive(Clone, Serialize, Deserialize, Union)]
pub enum Workflowable {
    Candidacy(Candidacy),
}

impl Workflowable {
    pub fn id(&self) -> WorkflowableId {
        match self {
            Self::Candidacy(inner) => inner.id.into(),
        }
    }
}

impl From<WorkflowableRow> for Workflowable {
    fn from(item: WorkflowableRow) -> Self {
        None.or_else(|| item.1.clone().map(Self::Candidacy))
            .unwrap()
    }
}
