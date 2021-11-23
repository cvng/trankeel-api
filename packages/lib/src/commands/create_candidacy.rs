use crate::candidacies;
use crate::candidacies::CreateCandidacyPayload;
use crate::candidacies::CreateCandidacyState;
use crate::client::Context;
use crate::templates::CandidacyCreatedMail;
use crate::Command;
use crate::CreateCandidacyInput;
use crate::Result;
use trankeel_core::activity::trace;
use trankeel_core::activity::Trace;
use trankeel_core::database::Db;
use trankeel_core::error::Error;
use trankeel_core::mailer::Mailer;

pub(crate) struct CreateCandidacy<'a> {
    context: &'a Context,
}

impl<'a> CreateCandidacy<'a> {
    pub fn new(context: &'a Context) -> Self {
        Self { context }
    }
}

#[async_trait]
impl<'a> Command for CreateCandidacy<'a> {
    type Input = CreateCandidacyInput;
    type Payload = CreateCandidacyPayload;

    async fn run(&self, input: Self::Input) -> Result<Self::Payload> {
        let db = self.context.db();
        let mailer = self.context.mailer();

        let account = db.accounts().by_advertisement_id(&input.advertisement_id)?;
        // In the context of a candidacy, the recipient is the account owner.
        let account_owner = db
            .persons()
            .by_account_id(&account.id)?
            .first()
            .cloned()
            .ok_or_else(|| Error::msg("recipient not found"))?;

        let state = CreateCandidacyState {
            account,
            account_owner,
        };

        let payload = candidacies::create_candidacy(state, input)?;

        db.transaction(|| {
            db.persons().create(&payload.candidate)?;
            db.candidacies().create(&payload.candidacy)?;
            if let Some(warrants) = &payload.warrants {
                db.warrants().create_many(warrants)?;
            }
            db.discussions().create(&payload.discussion)?;
            db.messages().create_many(&payload.messages)?;
            Ok(())
        })?;

        trace(db, Trace::CandidacyCreated(payload.candidacy.clone()))?;

        mailer
            .batch(vec![CandidacyCreatedMail::try_new(
                &payload.candidacy,
                &payload.candidate,
            )?])
            .await?;

        Ok(payload)
    }
}
