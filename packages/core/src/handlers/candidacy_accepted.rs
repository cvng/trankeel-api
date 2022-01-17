use super::candidacy_rejected;
use super::lease_affected;
use super::step_completed;
use super::StepCompletedHandler;
use super::StepCompletedPayload;
use crate::context::Context;
use crate::database::Db;
use crate::dispatcher::Handler;
use crate::error::Result;
use crate::mailer::Mailer;
use crate::messenger::Messenger;
use crate::pdfmaker::Pdfmaker;
use crate::templates::CandidacyAcceptedMail;
use crate::templates::LeaseDocument;
use chrono::Utc;
use diesel::result::Error::NotFound;
use trankeel_data::workflow_steps;
use trankeel_data::EventType;
use trankeel_data::Eventable;
use trankeel_data::LeaseFile;
use trankeel_data::StepEvent;
use trankeel_ops::event::CandidacyAccepted;
use trankeel_ops::event::CandidacyRejected;
use trankeel_ops::event::LeaseAffected;
use trankeel_ops::event::StepCompleted;

pub fn candidacy_accepted(ctx: &Context, event: CandidacyAccepted) -> Result<()> {
    let db = ctx.db();

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

    let steps = workflow_steps(&workflow);

    // Take first step from workflow (candidacy_accepted).
    let candidacy_accepted_step = steps
        .iter()
        .find(|step| step.as_event() == Some(StepEvent::CandidacyAccepted))
        .cloned();

    // Mark step as completed if found.
    let candidacy_accepted_step = if let Some(step) = candidacy_accepted_step {
        StepCompletedHandler::new(&step, &lease, &discussion)
            .run(StepCompleted {
                step_id: step.id,
                requirements: None,
            })
            .map(
                |StepCompletedPayload {
                     step,
                     lease: _lease,
                     discussion: _discussion,
                 }| Some(step),
            )?
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
    for step in steps {
        db.steps().create(&step)?;
    }
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
        step_completed(
            ctx,
            StepCompleted {
                step_id: step.id,
                requirements: None,
            },
        )?;
    }

    Ok(())
}

pub async fn candidacy_accepted_async(ctx: &Context, event: CandidacyAccepted) -> Result<()> {
    let db = ctx.db();
    let mailer = ctx.mailer();
    let pdfmaker = ctx.pdfmaker();
    let messenger = ctx.messenger();

    let account = db.accounts().by_candidacy_id(&event.candidacy.id)?;
    let participant = db.persons().by_candidacy_id(&event.candidacy.id)?;
    let sender = db
        .persons()
        .by_account_id(&account.id)?
        .first()
        .cloned()
        .ok_or(NotFound)?;

    messenger.message(
        EventType::CandidacyAccepted,
        Eventable::Candidacy(event.candidacy.clone()),
        sender.id,
        participant.id,
        None,
    )?;

    // Generate lease document (PDF) and assign document external ID to lease file.
    let document = pdfmaker
        .generate(LeaseDocument::try_new(
            &event.lease,
            &event.lease_file,
            Utc::now().into(),
        )?)
        .await?;

    db.files().update(&LeaseFile {
        external_id: Some(document.id),
        ..event.lease_file
    })?;

    mailer
        .batch(vec![CandidacyAcceptedMail::try_new(
            &event.candidacy,
            &event.identity,
            &event.invite,
        )?])
        .await?;

    Ok(())
}
