use crate::candidacies;
use crate::candidacies::RejectCandidacyInput;
use crate::candidacies::RejectCandidacyPayload;
use crate::candidacies::RejectCandidacyState;
use crate::client::Context;
use crate::Command;
use crate::Result;
use trankeel_core::activity::trace;
use trankeel_core::activity::Trace;
use trankeel_core::database::Db;
use trankeel_data::AuthId;

pub(crate) struct RejectCandidacy<'a> {
    context: &'a Context,
    auth_id: &'a AuthId,
}

impl<'a> RejectCandidacy<'a> {
    pub fn new(context: &'a Context, auth_id: &'a AuthId) -> Self {
        Self { context, auth_id }
    }
}

impl<'a> Command for RejectCandidacy<'a> {
    type Input = RejectCandidacyInput;
    type Payload = RejectCandidacyPayload;

    fn run(&self, input: Self::Input) -> Result<Self::Payload> {
        let db = self.context.db();

        let candidacy = db.candidacies().by_id(&input.id)?;
        let discussion = db.discussions().by_candidacy_id(&candidacy.id)?;
        let account_owner = db.persons().by_auth_id(self.auth_id)?;
        let candidate = db.persons().by_candidacy_id(&candidacy.id)?;

        let state = RejectCandidacyState {
            candidacy,
            discussion,
            account_owner,
            candidate,
        };

        let payload = candidacies::reject_candidacy(state, input)?;

        db.transaction(|| {
            db.candidacies().update(&payload.candidacy)?;
            db.discussions().update(&payload.discussion)?;
            db.messages().create(&payload.message)?;
            Ok(())
        })?;

        trace(db, Trace::CandidacyRejected(payload.candidacy.clone()))?;

        Ok(payload)
    }
}
