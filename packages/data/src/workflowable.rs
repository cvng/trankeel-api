use crate::Candidacy;
use crate::Id;
use crate::Workflow;

pub type WorkflowableId = Id;

pub type WorkflowableRow = (Workflow, Option<Candidacy>);

#[derive(Clone, Union)]
pub enum Workflowable {
    Candidacy(Candidacy),
}

impl Workflowable {
    pub fn id(&self) -> WorkflowableId {
        match self {
            Self::Candidacy(inner) => inner.id,
        }
    }
}

impl From<WorkflowableRow> for Workflowable {
    fn from(item: WorkflowableRow) -> Self {
        None.or_else(|| item.1.clone().map(Self::Candidacy))
            .unwrap()
    }
}
