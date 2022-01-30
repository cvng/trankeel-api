use crate::error::Result;
use crate::event::Event;
use crate::event::NoticeCreated;
use crate::Command;
use async_graphql::InputObject;
use trankeel_data::DateTime;
use trankeel_data::Notice;
use trankeel_data::Rent;
use trankeel_data::RentId;
use validator::Validate;

#[derive(InputObject, Validate)]
pub struct CreateNoticesInput {
    pub rent_ids: Vec<RentId>,
    pub date: Option<DateTime>,
}

pub struct CreateNotices {
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

    fn run(self, input: Self::Input) -> Result<Vec<Event>> {
        input.validate()?;

        let Self { rents } = self;

        let notices = rents.into_iter().map(|rent| {
            // Create new notice.
            let notice = Notice::notice_document(&rent);

            // Link notice with rent.
            let rent = Rent {
                notice_id: Some(notice.id),
                ..rent
            };

            (notice, rent)
        });

        Ok(notices
            .map(|(notice, rent)| NoticeCreated { notice, rent }.into())
            .collect::<Vec<_>>())
    }
}
