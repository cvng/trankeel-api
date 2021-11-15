use super::reject_candidacy;
use super::RejectCandidacyInput;
use crate::client::Actor;
use crate::client::Context;
use crate::error::no;
use crate::error::Result;
use crate::invites::create_invite;
use crate::invites::CreateInviteInput;
use crate::leases::create_lease_from_advertisement;
use crate::ops;
use crate::templates::CandidacyAcceptedMail;
use crate::templates::LeaseDocument;
use crate::workflows::complete_step;
use crate::workflows::create_workflow;
use crate::workflows::CreateWorkflowInput;
use crate::CompleteStepInput;
use crate::CreateTenantInput;
use async_graphql::InputObject;
use chrono::Utc;
use trankeel_core::activity::trace;
use trankeel_core::activity::Trace;
use trankeel_core::database::Db;
use trankeel_core::mailer::Mailer;
use trankeel_core::pdfmaker::Pdfmaker;
use trankeel_data::lease_filename;
use trankeel_data::Candidacy;
use trankeel_data::CandidacyId;
use trankeel_data::CandidacyStatus;
use trankeel_data::FileType;
use trankeel_data::InviteReason;
use trankeel_data::Lease;
use trankeel_data::LeaseFile;
use trankeel_data::LeaseFileId;
use trankeel_data::Person;
use trankeel_data::PersonRole;
use trankeel_data::WorkflowType;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
pub struct AcceptCandidacyInput {
    pub id: CandidacyId,
}

// # Operation

pub(crate) async fn accept_candidacy(
    ctx: &Context,
    actor: &Actor,
    input: AcceptCandidacyInput,
) -> Result<Candidacy> {
    let db = ctx.db();
    let pdfmaker = ctx.pdfmaker();
    let mailer = ctx.mailer();
    let auth_id = actor.check()?;

    input.validate()?;

    let advertisement = db.advertisements().by_candidacy_id(&input.id)?;

    // Reject other candidacies.
    let other_candidacies = db
        .candidacies()
        .by_advertisement_id(&advertisement.id)?
        .into_iter()
        .filter(|candidacy| candidacy.id != input.id)
        .collect::<Vec<Candidacy>>();

    for candidacy in other_candidacies {
        reject_candidacy(ctx, actor, RejectCandidacyInput { id: candidacy.id }).await?;
    }

    // Accept given candidacy.
    let candidacy = db.candidacies().by_id(&input.id)?;

    let candidacy = db.candidacies().update(&Candidacy {
        id: input.id,
        status: CandidacyStatus::Accepted,
        ..candidacy
    })?;

    trace(db, Trace::CandidacyAccepted(candidacy.clone()))?;

    // Create tenant profile.
    let candidate = db.persons().by_candidacy_id(&candidacy.id)?;
    let candidacy_warrants = db.warrants().by_candidacy_id(&candidacy.id)?;

    let tenant = ops::create_tenant(
        ctx,
        actor,
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
            person_id: Some(candidate.id),
        },
    )?
    .tenant;
    db.persons().update(&Person {
        id: candidate.id,
        role: PersonRole::Tenant,
        ..candidate.clone()
    })?;

    // Send invite to candidate.
    let invite = create_invite(
        db,
        CreateInviteInput {
            invitee_id: candidate.id,
            reason: InviteReason::CandidacyAccepted,
        },
    )?;

    // Create unsigned lease.
    let advertisement = db.advertisements().by_id(&candidacy.advertisement_id)?;
    let lease = create_lease_from_advertisement(db, auth_id, &advertisement, vec![tenant])?;

    // Init new lease file.
    let lease_file_id = LeaseFileId::new();
    let mut lease_file = LeaseFile {
        id: lease_file_id,
        type_: FileType::LeaseDocument,
        filename: Some(lease_filename(&lease_file_id, &lease)),
        status: None,
        external_id: None,
        download_url: None,
        preview_url: None,
        created_at: None,
        updated_at: None,
    };

    // Try to generate lease document (PDF).
    let document = LeaseDocument::try_new(lease.clone(), lease_file.clone(), Utc::now().into())?;
    let document = pdfmaker.generate(document).await?;

    // Assign lease file external ID.
    lease_file.external_id = Some(document.id);
    lease_file.status = Some(document.status);

    // Create lease file.
    let lease_file = match db.files().create(&lease_file) {
        Ok(lease_file) => lease_file,
        Err(err) => return Err(err),
    };

    // Link lease file with lease.
    db.leases().update(&Lease {
        id: lease.id,
        lease_id: Some(lease_file.id),
        ..lease
    })?;

    // Init candidacy workflow.
    let workflow = create_workflow(
        db,
        CreateWorkflowInput {
            type_: WorkflowType::Candidacy,
            workflowable_id: candidacy.id,
        },
    )?;

    // Mark first step as completed (candidacy_accepted)
    let steps = db.steps().by_workflow_id(&workflow.id)?;

    complete_step(
        db,
        CompleteStepInput {
            id: steps.first().ok_or_else(|| no("workflow.first_step"))?.id,
            requirements: None,
        },
    )?;

    mailer
        .batch(vec![CandidacyAcceptedMail::try_new(
            &candidacy, &candidate, &invite,
        )?])
        .await?;

    Ok(candidacy)
}
