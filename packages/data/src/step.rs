use crate::id;
use crate::sql_schema::steps;
use crate::DateTime;
use crate::RequirementOuter;
use crate::Workflow;
use crate::WorkflowId;
use async_graphql::SimpleObject;
use fake::Fake;

id!(StepId);

#[derive(PartialEq)]
pub enum StepEvent {
    CandidacyAccepted,
    LeaseSigned,
    LeaseConfirmed,
    LeaseActivated,
    InsuranceOk,
    DepositAmountOk,
}

impl From<String> for StepEvent {
    fn from(item: String) -> Self {
        match item.as_str() {
            "candidacy_accepted" => Self::CandidacyAccepted,
            "lease_signed" => Self::LeaseSigned,
            "lease_confirmed" => Self::LeaseConfirmed,
            "lease_activated" => Self::LeaseActivated,
            "insurance_ok" => Self::InsuranceOk,
            "deposit_amount_ok" => Self::DepositAmountOk,
            _ => unimplemented!(),
        }
    }
}

#[derive(
    Clone, Debug, Associations, Identifiable, Insertable, Queryable, SimpleObject, AsChangeset,
)]
#[belongs_to(parent = "Workflow")]
pub struct Step {
    pub id: StepId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub workflow_id: WorkflowId,
    pub label: String,
    pub completed: bool,
    pub confirmation: Option<String>,
    pub requirements: Option<RequirementOuter>,
    pub event: Option<String>,
}

impl Step {
    pub fn as_event(&self) -> Option<StepEvent> {
        self.event.clone().map(Into::into)
    }
}
