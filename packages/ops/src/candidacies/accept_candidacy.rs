use crate::error::Result;
use crate::event::CandidacyAccepted;
use crate::event::CandidacyRejected;
use crate::event::Event;
use crate::event::LeaseAffected;
use crate::event::LeaseCreated;
use crate::event::LeaseFileRequested;
use crate::event::StepCompleted;
use crate::event::StepCreated;
use crate::event::TenantCreated;
use crate::event::WorkflowCreated;
use crate::leases::CreateLease;
use crate::leases::CreateLeaseInput;
use crate::tenants::CreateTenant;
use crate::tenants::CreateTenantInput;
use crate::workflows::CreateWorkflow;
use crate::workflows::CreateWorkflowInput;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::workflow_steps;
use trankeel_data::Account;
use trankeel_data::Advertisement;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyId;
use trankeel_data::CandidacyStatus;
use trankeel_data::Discussion;
use trankeel_data::LeaseId;
use trankeel_data::LeaseType;
use trankeel_data::Person;
use trankeel_data::StepEvent;
use trankeel_data::TenantId;
use trankeel_data::WarrantWithIdentity;
use trankeel_data::WorkflowType;
use trankeel_data::Workflowable;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct AcceptCandidacyInput {
    pub id: CandidacyId,
}

pub struct AcceptCandidacy {
    candidacy: Candidacy,
    account: Account,
    account_owner: Person,
    advertisement: Advertisement,
    candidacy_warrants: Vec<WarrantWithIdentity>,
    candidate: Person,
    discussion: Discussion,
    other_candidacies: Vec<Candidacy>,
}

impl AcceptCandidacy {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        candidacy: &Candidacy,
        account: &Account,
        account_owner: &Person,
        advertisement: &Advertisement,
        candidacy_warrants: &[WarrantWithIdentity],
        candidate: &Person,
        discussion: &Discussion,
        other_candidacies: &[Candidacy],
    ) -> Self {
        Self {
            candidacy: candidacy.clone(),
            account: account.clone(),
            account_owner: account_owner.clone(),
            advertisement: advertisement.clone(),
            candidacy_warrants: candidacy_warrants.to_vec(),
            candidate: candidate.clone(),
            discussion: discussion.clone(),
            other_candidacies: other_candidacies.to_vec(),
        }
    }
}

impl Command for AcceptCandidacy {
    type Input = AcceptCandidacyInput;

    fn run(self, input: Self::Input) -> Result<Vec<Event>> {
        input.validate()?;

        let Self {
            candidacy,
            account,
            account_owner,
            advertisement,
            candidacy_warrants,
            candidate,
            discussion: _discussion,
            other_candidacies,
        } = self;

        // Make given candidacy accepted.
        let candidacy = Candidacy {
            status: CandidacyStatus::Accepted,
            ..candidacy
        };

        // Make other candidacies rejected.
        let rejected_candidacies = other_candidacies;

        // Create tenant with identity.
        let (
            tenant,
            _identity,
            warrants,
            _discussion, //
        ) = CreateTenant::new(TenantId::new(), &account, &account_owner, Some(&candidate))
            .run(CreateTenantInput {
                birthdate: candidacy.birthdate,
                birthplace: candidacy.birthplace.clone(),
                email: candidate.email.inner().to_string(),
                first_name: candidate.first_name.clone(),
                last_name: candidate.last_name.clone(),
                note: None,
                phone_number: candidate.phone_number.clone(),
                is_student: candidacy.is_student,
                warrants: Some(candidacy_warrants.into_iter().map(Into::into).collect()),
            })?
            .into_iter()
            .find_map(|event| match event {
                Event::TenantCreated(event) => Some((
                    event.tenant,
                    event.identity,
                    event.warrants,
                    event.discussion,
                )),
                _ => None,
            })
            .unwrap();

        // Create unsigned lease from advertisement.
        let (lease, rents) = CreateLease::new(LeaseId::new(), &account, &vec![tenant.clone()])
            .run(CreateLeaseInput {
                details: None,
                deposit_amount: Some(advertisement.deposit_amount),
                effect_date: advertisement.effect_date,
                renew_date: None,
                type_: LeaseType::default(),
                property_id: advertisement.property_id,
                rent_amount: advertisement.rent_amount,
                rent_charges_amount: advertisement.rent_charges_amount,
                signature_date: None,
                tenant_ids: vec![tenant.id],
            })?
            .into_iter()
            .find_map(|event| match event {
                Event::LeaseCreated(event) => Some((event.lease, event.rents)),
                _ => None,
            })
            .unwrap();

        // Create workflowable.
        let workflowable = Workflowable::Candidacy(candidacy.clone());

        // Setup candidacy workflow.
        let workflow = CreateWorkflow::new(&workflowable) //
            .run(CreateWorkflowInput {
                type_: WorkflowType::Candidacy,
                workflowable_id: candidacy.id.into(),
            })?
            .into_iter()
            .find_map(|event| match event {
                Event::WorkflowCreated(event) => Some(event.workflow),
                _ => None,
            })
            .unwrap();

        let steps = workflow_steps(&workflow);

        // Take first step from workflow (candidacy_accepted).
        let candidacy_accepted_step = steps
            .iter()
            .find(|step| step.as_event() == Some(StepEvent::CandidacyAccepted))
            .cloned()
            .unwrap();

        Ok(vec![
            CandidacyAccepted {
                candidacy_id: candidacy.id,
            }
            .into(),
            TenantCreated {
                tenant: tenant.clone(),
                identity: None,
                warrants,
                discussion: None,
            }
            .into(),
            LeaseCreated {
                lease: lease.clone(),
                rents,
            }
            .into(),
            LeaseAffected {
                lease_id: lease.id,
                tenant_id: tenant.id,
            }
            .into(),
            LeaseFileRequested { lease_id: lease.id }.into(),
            WorkflowCreated {
                workflow,
                workflowable,
            }
            .into(),
        ]
        .into_iter()
        .chain(
            steps
                .into_iter()
                .map(|step| StepCreated { step }.into())
                .collect::<Vec<_>>(),
        )
        .chain(vec![StepCompleted {
            step_id: candidacy_accepted_step.id,
            requirements: None,
        }
        .into()])
        .chain(
            rejected_candidacies
                .into_iter()
                .map(|candidacy| {
                    CandidacyRejected {
                        candidacy_id: candidacy.id,
                    }
                    .into()
                })
                .collect::<Vec<_>>(),
        )
        .collect())
    }
}
