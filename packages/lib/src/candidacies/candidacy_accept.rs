use super::RejectCandidacy;
use super::RejectCandidacyInput;
use super::RejectCandidacyPayload;
use crate::error::Result;
use crate::invites::create_invite;
use crate::invites::CreateInviteInput;
use crate::leases::create_lease_from_advertisement;
use crate::tenants::CreateTenant;
use crate::workflows::complete_step;
use crate::workflows::create_workflow;
use crate::workflows::CreateWorkflowInput;
use crate::CompleteStepInput;
use crate::CreateTenantInput;
use crate::CreateTenantPayload;
use async_graphql::InputObject;
use chrono::Utc;
use trankeel_core::context::Context;
use trankeel_core::database::Db;
use trankeel_core::dispatcher;
use trankeel_core::dispatcher::dispatch;
use trankeel_core::dispatcher::Command;
use trankeel_core::dispatcher::Event;
use trankeel_core::error::no;
use trankeel_core::handlers::CandidacyRejected;
use trankeel_core::mailer::Mailer;
use trankeel_core::pdfmaker::Pdfmaker;
use trankeel_core::templates::CandidacyAcceptedMail;
use trankeel_core::templates::LeaseDocument;
use trankeel_data::lease_filename;
use trankeel_data::Account;
use trankeel_data::AuthId;
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
use trankeel_data::Tenant;
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
    auth_id: &AuthId,
    input: AcceptCandidacyInput,
) -> Result<Candidacy> {
    let db = ctx.db();
    let pdfmaker = ctx.pdfmaker();
    let mailer = ctx.mailer();

    input.validate()?;

    let account = db.accounts().by_auth_id(auth_id)?;
    let account_owner = db.persons().by_auth_id(auth_id)?;

    let advertisement = db.advertisements().by_candidacy_id(&input.id)?;

    // Reject other candidacies.
    let other_candidacies = db
        .candidacies()
        .by_advertisement_id(&advertisement.id)?
        .into_iter()
        .filter(|candidacy| candidacy.id != input.id)
        .collect::<Vec<Candidacy>>();

    for candidacy in other_candidacies {
        let account_owner = db.persons().by_auth_id(auth_id)?;
        let candidate = db.persons().by_candidacy_id(&candidacy.id)?;
        let discussion = db.discussions().by_candidacy_id(&candidacy.id)?;
        RejectCandidacy::new(&candidacy, &candidate, &account_owner, &discussion)
            .run(RejectCandidacyInput { id: candidacy.id })
            .and_then(
                |RejectCandidacyPayload {
                     candidacy,
                     discussion,
                     message,
                 }| {
                    dispatcher::dispatch(
                        ctx,
                        vec![CandidacyRejected {
                            candidacy,
                            discussion,
                            message,
                        }
                        .into()],
                    )
                },
            )?;
    }

    // Accept given candidacy.
    let candidacy = db.candidacies().by_id(&input.id)?;

    let candidacy = db.candidacies().update(&Candidacy {
        id: input.id,
        status: CandidacyStatus::Accepted,
        ..candidacy
    })?;

    dispatch(ctx, vec![Event::CandidacyAccepted(candidacy.clone())])?;

    // Create tenant profile.
    let candidate = db.persons().by_candidacy_id(&candidacy.id)?;

    let tenant = promote_tenant(ctx, &account, &account_owner, &candidacy, &candidate)?;
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
    let lease = create_lease_from_advertisement(ctx, auth_id, &advertisement, vec![tenant])?;

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
        ctx,
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

// # Utils

fn promote_tenant(
    ctx: &Context,
    account: &Account,
    account_owner: &Person,
    candidacy: &Candidacy,
    candidate: &Person,
) -> Result<Tenant> {
    let db = ctx.db();

    let candidacy_warrants = db.warrants().by_candidacy_id(&candidacy.id)?;

    let CreateTenantPayload {
        tenant,
        identity,
        warrants,
        discussion,
    } = CreateTenant::new(account, account_owner, Some(candidate)).run(CreateTenantInput {
        birthdate: candidacy.birthdate,
        birthplace: candidacy.birthplace.clone(),
        email: candidate.email.inner().to_string(),
        first_name: candidate.first_name.clone(),
        last_name: candidate.last_name.clone(),
        note: None,
        phone_number: candidate.phone_number.clone(),
        is_student: candidacy.is_student,
        warrants: Some(candidacy_warrants.into_iter().map(Into::into).collect()),
    })?;

    ctx.db().persons().create(&identity)?;
    ctx.db().tenants().create(&tenant)?;
    if let Some(warrants) = &warrants {
        for warrant in warrants {
            ctx.db().warrants().create(warrant)?;
        }
    }
    if let Some(discussion) = &discussion {
        ctx.db().discussions().create(discussion)?;
    }

    Ok(tenant)
}
