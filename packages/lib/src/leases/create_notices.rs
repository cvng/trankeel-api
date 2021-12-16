use crate::error::Result;
use async_graphql::InputObject;
use trankeel_core::dispatcher::Command;
use trankeel_data::DateTime;
use trankeel_data::Notice;
use trankeel_data::NoticeWithRent;
use trankeel_data::Rent;
use trankeel_data::RentId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateNoticesInput {
    pub rent_ids: Vec<RentId>,
    pub date: Option<DateTime>,
}

pub struct CreateNoticesPayload {
    pub notices: Vec<NoticeWithRent>,
}

pub(crate) struct CreateNotices {
    rents: Vec<Rent>,
}

impl CreateNotices {
    pub fn new(rents: &[Rent]) -> Self {
        Self {
            rents: rents.to_vec(),
        }
    }
}

impl Command for CreateNotices {
    type Input = CreateNoticesInput;
    type Payload = CreateNoticesPayload;

    fn run(self, input: Self::Input) -> Result<Self::Payload> {
        input.validate()?;

        let Self { rents } = self;

        let notices = rents
            .into_iter()
            .map(|rent| {
                // Create new notice.
                let notice = Notice::notice_document(&rent);

                // Link notice with rent.
                let rent = Rent {
                    notice_id: Some(notice.id),
                    ..rent
                };

                (notice, rent)
            })
            .collect();

        Ok(Self::Payload { notices })
    }
}
