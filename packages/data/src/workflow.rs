use crate::id;
use crate::sql_schema::workflows;
use crate::DateTime;
use crate::RequirementOuter;
use crate::Step;
use crate::StepId;
use crate::WorkflowableId;
use async_graphql::Enum;
use diesel_derive_enum::DbEnum;
use fake::Fake;
use serde::Deserialize;
use serde::Serialize;
use trankeel_kit::config;

id!(WorkflowId);

pub type WorkflowWithSteps = (Workflow, Vec<Step>);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DbEnum, Enum)]
#[DieselType = "Workflowtype"]
#[serde(rename_all = "snake_case")]
pub enum WorkflowType {
    Candidacy,
}

#[derive(Clone, Debug, Serialize, Identifiable, Insertable, Queryable)]
pub struct Workflow {
    pub id: WorkflowId,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    pub workflowable_id: WorkflowableId,
    pub type_: WorkflowType,
    pub completed: bool,
}

#[derive(Default, AsChangeset, Identifiable, Insertable)]
#[table_name = "workflows"]
pub struct WorkflowData {
    pub id: WorkflowId,
    pub workflowable_id: Option<WorkflowableId>,
    pub type_: Option<WorkflowType>,
    pub completed: Option<bool>,
}

pub fn workflow_steps(workflow: &Workflow) -> Vec<Step> {
    config::config()
        .workflows(&serde_json::to_string(&workflow.type_).unwrap())
        .unwrap()
        .parse()
        .into_iter()
        .map(|step| Step {
            id: StepId::new(),
            created_at: Default::default(),
            updated_at: Default::default(),
            workflow_id: workflow.id,
            label: step.label,
            event: Some(step.event),
            completed: Default::default(),
            confirmation: Some(step.confirmation),
            requirements: step.requirements.map(|requirements| RequirementOuter {
                requirements: requirements.into_iter().map(Into::into).collect(),
            }),
        })
        .collect()
}
