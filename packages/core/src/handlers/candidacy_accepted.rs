use super::candidacy_rejected;
use super::lease_affected;
use super::step_completed;
use super::CandidacyRejected;
use super::LeaseAffected;
use super::StepCompleted;
use crate::config;
use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Event;
use crate::error::Result;
use crate::mailer::Mailer;
use crate::messenger::Messenger;
use crate::pdfmaker::Pdfmaker;
use crate::templates::CandidacyAcceptedMail;
use crate::templates::LeaseDocument;
use chrono::Utc;
use diesel::result::Error::NotFound;
use trankeel_data::Candidacy;
use trankeel_data::Discussion;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::Invite;
use trankeel_data::Lease;
use trankeel_data::LeaseFile;
use trankeel_data::Message;
use trankeel_data::Person;
use trankeel_data::Rent;
use trankeel_data::StepEvent;
use trankeel_data::Tenant;
use trankeel_data::WarrantWithIdentity;
use trankeel_data::Workflow;
use trankeel_data::Workflowable;
use trankeel_ops::workflows::CompleteStep;
use trankeel_ops::workflows::CompleteStepInput;
use trankeel_ops::workflows::CompleteStepPayload;
use trankeel_ops::Command;

#[derive(Clone)]
pub struct CandidacyAccepted {
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
    pub invite: Invite,
}

impl From<CandidacyAccepted> for Event {
    fn from(item: CandidacyAccepted) -> Self {
        Self::CandidacyAccepted(item)
    }
}

pub fn candidacy_accepted(ctx: &Context, event: CandidacyAccepted) -> Result<()> {
    let db = ctx.db();
    let messenger = ctx.messenger();

    let CandidacyAccepted {
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
        invite: _invite,
    } = event;

    let steps = config::workflow_steps(&workflow);

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

    db.candidacies().update(&candidacy)?;
    db.persons().update(&identity)?;
    db.files().create(&lease_file)?;
    db.leases().create(&lease)?;
    db.rents().create_many(&rents)?;
    db.tenants().create(&tenant)?;
    db.discussions().update(&discussion)?;
    db.workflowables().create(&workflowable)?;
    db.workflows().create(&workflow)?;
    db.steps().create_many(&steps)?;
    if let Some(warrants) = &warrants {
        db.warrants().create_many(warrants)?;
    }

    lease_affected(ctx, LeaseAffected { tenant })?;

    for (candidacy, (discussion, message)) in rejected_candidacies {
        candidacy_rejected(
            ctx,
            CandidacyRejected {
                candidacy,
                discussion,
                message,
            },
        )?;
    }

    if let Some(step) = candidacy_accepted_step {
        step_completed(ctx, StepCompleted { step })?;
    }

    let account = db.accounts().by_candidacy_id(&candidacy.id)?;
    let participant = db.persons().by_candidacy_id(&candidacy.id)?;
    let sender = db
        .persons()
        .by_account_id(&account.id)?
        .first()
        .cloned()
        .ok_or(NotFound)?;

    messenger.message(
        EventType::CandidacyAccepted,
        Eventable::Candidacy(candidacy),
        sender.id,
        participant.id,
        None,
    )?;

    Ok(())
}

pub async fn candidacy_accepted_async(ctx: &Context, event: CandidacyAccepted) -> Result<()> {
    let db = ctx.db();
    let mailer = ctx.mailer();
    let pdfmaker = ctx.pdfmaker();

    let CandidacyAccepted {
        candidacy,
        identity,
        lease,
        lease_file,
        invite,
        ..
    } = event;

    // Generate lease document (PDF) and assign document external ID to lease file.
    let document = pdfmaker
        .generate(LeaseDocument::try_new(
            &lease,
            &lease_file,
            Utc::now().into(),
        )?)
        .await?;

    db.files().update(&LeaseFile {
        external_id: Some(document.id),
        ..lease_file
    })?;

    mailer
        .batch(vec![CandidacyAcceptedMail::try_new(
            &candidacy, &identity, &invite,
        )?])
        .await?;

    Ok(())
}
