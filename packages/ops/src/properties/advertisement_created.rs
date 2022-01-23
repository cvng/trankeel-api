use crate::event::AdvertisementCreated;
use crate::state::State;

pub fn advertisement_created(state: State, event: AdvertisementCreated) -> State {
    State {
        advertisements: vec![event.advertisement],
        ..state
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use trankeel_data::fake::Fake;
    use trankeel_data::fake::Faker;

    #[test]
    fn test_advertisement_created() {
        let state = State {
            advertisements: Vec::new(),
            ..Default::default()
        };

        let event = AdvertisementCreated {
            advertisement: Faker.fake(),
        };

        assert_eq!(
            advertisement_created(state, event.clone())
                .advertisements
                .first()
                .unwrap()
                .id,
            State {
                advertisements: vec![event.advertisement],
                ..Default::default()
            }
            .advertisements
            .first()
            .unwrap()
            .id
        );
    }
}
