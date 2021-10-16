use super::reject_candidacy;
use super::RejectCandidacyInput;
use crate::error::Result;
use crate::invites::create_invite;
use crate::invites::CreateInviteInput;
use crate::templates::CandidacyAcceptedMail;
use async_graphql::InputObject;
use piteo_core::activity::trace;
use piteo_core::activity::Trace;
use piteo_core::database::Db;
use piteo_core::mailer::Mailer;
use piteo_data::AuthId;
use piteo_data::Candidacy;
use piteo_data::CandidacyData;
use piteo_data::CandidacyId;
use piteo_data::CandidacyStatus;
use piteo_data::InviteReason;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
pub struct AcceptCandidacyInput {
    pub id: CandidacyId,
}

// # Operation

pub async fn accept_candidacy(
    db: &impl Db,
    mailer: &impl Mailer,
    auth_id: &AuthId,
    input: AcceptCandidacyInput,
) -> Result<Candidacy> {
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
        reject_candidacy(db, auth_id, RejectCandidacyInput { id: candidacy.id }).await?;
    }

    // Accept given candidacy.
    let candidacy = db.candidacies().update(CandidacyData {
        id: input.id,
        status: Some(CandidacyStatus::Accepted),
        ..Default::default()
    })?;

    trace(db, Trace::CandidacyAccepted(candidacy.clone())).ok();

    let candidate = db.persons().by_candidacy_id(&candidacy.id)?;

    let invite = create_invite(
        db,
        CreateInviteInput {
            id: candidate.id,
            reason: InviteReason::CandidacyAccepted,
        },
    )?;

    mailer
        .batch(vec![CandidacyAcceptedMail::try_new(
            &candidacy, &candidate, &invite,
        )?])
        .await?;

    Ok(candidacy)
}
