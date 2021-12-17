use super::RejectCandidacy;
use super::RejectCandidacyInput;
use super::RejectCandidacyPayload;
use crate::error::Result;
use crate::invites::CreateInvite;
use crate::invites::CreateInviteInput;
use crate::invites::CreateInvitePayload;
use crate::leases::CreateFurnishedLease;
use crate::leases::CreateFurnishedLeaseInput;
use crate::leases::CreateFurnishedLeasePayload;
use crate::tenants::CreateTenant;
use crate::tenants::CreateTenantInput;
use crate::tenants::CreateTenantPayload;
use crate::workflows::CompleteStep;
use crate::workflows::CompleteStepInput;
use crate::workflows::CompleteStepPayload;
use crate::workflows::CreateWorkflow;
use crate::workflows::CreateWorkflowInput;
use crate::workflows::CreateWorkflowPayload;
use async_graphql::InputObject;
use trankeel_core::dispatcher::Command;
use trankeel_data::Account;
use trankeel_data::Advertisement;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyId;
use trankeel_data::CandidacyStatus;
use trankeel_data::Discussion;
use trankeel_data::DiscussionStatus;
use trankeel_data::Invite;
use trankeel_data::InviteReason;
use trankeel_data::Lease;
use trankeel_data::LeaseFile;
use trankeel_data::Message;
use trankeel_data::Person;
use trankeel_data::Rent;
use trankeel_data::Step;
use trankeel_data::StepEvent;
use trankeel_data::Tenant;
use trankeel_data::WarrantWithIdentity;
use trankeel_data::Workflow;
use trankeel_data::WorkflowType;
use trankeel_data::Workflowable;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct AcceptCandidacyInput {
    pub id: CandidacyId,
}

pub struct AcceptCandidacyPayload {
    pub candidacy: Candidacy,
    pub rejected_candidacies: Vec<(Candidacy, (Discussion, Message))>,
    pub tenant: Tenant,
    pub identity: Person,
    pub warrants: Option<Vec<WarrantWithIdentity>>,
    pub discussion: Discussion,
    pub lease: Lease,
    pub rents: Vec<Rent>,
    pub lease_file: LeaseFile,
    pub workflow: Workflow,
    pub workflowable: Workflowable,
    pub steps: Vec<Step>,
    pub candidacy_accepted_step: Option<Step>,
    pub invite: Invite,
}

pub struct AcceptCandidacy {
    candidacy: Candidacy,
    account: Account,
    account_owner: Person,
    advertisement: Advertisement,
    candidacy_warrants: Vec<WarrantWithIdentity>,
    candidate: Person,
    discussion: Discussion,
    other_candidacies: Vec<(Candidacy, Person, Discussion)>,
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
        other_candidacies: &[(Candidacy, Person, Discussion)],
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
    type Payload = AcceptCandidacyPayload;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self {
            candidacy,
            account,
            account_owner,
            advertisement,
            candidacy_warrants,
            candidate,
            discussion,
            other_candidacies,
        } = self;

        // Make given candidacy accepted.
        let candidacy = Candidacy {
            status: CandidacyStatus::Accepted,
            ..candidacy
        };

        // Make other candidacies rejected.
        let rejected_candidacies = other_candidacies
            .into_iter()
            .map(|(candidacy, candidate, discussion)| {
                RejectCandidacy::new(&candidacy, &candidate, &account_owner, &discussion)
                    .run(RejectCandidacyInput { id: candidacy.id })
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(
                |RejectCandidacyPayload {
                     candidacy,
                     discussion,
                     message,
                 }| (candidacy, (discussion, message)),
            )
            .collect::<Vec<_>>();

        // Create tenant with identity.
        let CreateTenantPayload {
            tenant,
            identity,
            warrants,
            discussion: _discussion,
        } = CreateTenant::new(&account, &account_owner, Some(&candidate)).run(
            CreateTenantInput {
                birthdate: candidacy.birthdate,
                birthplace: candidacy.birthplace.clone(),
                email: candidate.email.inner().to_string(),
                first_name: candidate.first_name.clone(),
                last_name: candidate.last_name.clone(),
                note: None,
                phone_number: candidate.phone_number.clone(),
                is_student: candidacy.is_student,
                warrants: Some(candidacy_warrants.into_iter().map(Into::into).collect()),
            },
        )?;

        // Make discussion active now.
        let discussion = Discussion {
            status: DiscussionStatus::Active,
            ..discussion
        };

        // Create invite for new tenant.
        let CreateInvitePayload { invite } = CreateInvite::new(&identity) //
            .run(CreateInviteInput {
                invitee_id: identity.id,
                reason: InviteReason::CandidacyAccepted,
            })?;

        // Create unsigned lease from advertisement.
        let CreateFurnishedLeasePayload {
            lease,
            rents,
            tenants,
        } = CreateFurnishedLease::new(&account, &vec![tenant.clone()]).run(
            CreateFurnishedLeaseInput {
                details: None,
                deposit_amount: advertisement.deposit_amount,
                effect_date: advertisement.effect_date,
                renew_date: None,
                file: None,
                property_id: advertisement.property_id,
                rent_amount: advertisement.rent_amount,
                rent_charges_amount: advertisement.rent_charges_amount,
                signature_date: None,
                tenant_ids: vec![tenant.id],
            },
        )?;

        // Take first tenant out of lease.
        let tenant = tenants.first().cloned().unwrap();

        // Create lease file.
        let lease_file = LeaseFile::lease_document(&lease);

        // Link lease file with lease.
        let lease = Lease {
            id: lease.id,
            lease_id: Some(lease_file.id),
            ..lease
        };

        // Create workflowable.
        let workflowable = Workflowable::Candidacy(candidacy.clone());

        // Setup candidacy workflow.
        let CreateWorkflowPayload { workflow, steps } = CreateWorkflow::new(&workflowable) //
            .run(CreateWorkflowInput {
                type_: WorkflowType::Candidacy,
                workflowable_id: candidacy.id,
            })?;

        // Take first step from workflow (candidacy_accepted).
        let candidacy_accepted_step = steps
            .iter()
            .find(|step| step.as_event() == Some(StepEvent::CandidacyAccepted))
            .cloned();

        // Mark step as completed if found.
        let candidacy_accepted_step = if let Some(step) = candidacy_accepted_step {
            CompleteStep::new(&step)
                .run(CompleteStepInput {
                    id: step.id,
                    requirements: None,
                })
                .map(|CompleteStepPayload { step }| Some(step))?
        } else {
            None
        };

        Ok(Self::Payload {
            candidacy,
            rejected_candidacies,
            tenant,
            identity,
            warrants,
            discussion,
            lease,
            rents,
            lease_file,
            workflow,
            workflowable,
            steps,
            candidacy_accepted_step,
            invite,
        })
    }
}
