use crate::client::Context;
use crate::properties;
use crate::properties::CreatePropertyPayload;
use crate::properties::CreatePropertyState;
use crate::Command;
use crate::CreatePropertyInput;
use crate::Result;
use trankeel_core::database::Db;
use trankeel_data::AuthId;

pub(crate) struct CreateProperty<'a> {
    context: &'a Context,
    auth_id: &'a AuthId,
}

impl<'a> CreateProperty<'a> {
    pub fn new(context: &'a Context, auth_id: &'a AuthId) -> Self {
        Self { context, auth_id }
    }
}

#[async_trait]
impl<'a> Command for CreateProperty<'a> {
    type Input = CreatePropertyInput;
    type Payload = CreatePropertyPayload;

    async fn run(&self, input: Self::Input) -> Result<Self::Payload> {
        let db = self.context.db();

        let state = CreatePropertyState {
            account: db.accounts().by_auth_id(self.auth_id)?,
        };

        let payload = properties::create_property(state, input)?;

        db.transaction(|| {
            db.properties().create(&payload.property)?;
            Ok(())
        })?;

        Ok(payload)
    }
}
