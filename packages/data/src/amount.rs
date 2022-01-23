use async_graphql::scalar;
use diesel_derive_newtype::DieselNewType;
use fake::Dummy;
use fake::Fake;
use fake::Faker;
use serde::Deserialize;
use serde::Serialize;

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, DieselNewType)]
pub struct Amount(rust_decimal::Decimal);

impl Amount {
    pub fn new(num: i64) -> Self {
        Self(rust_decimal::Decimal::new(num, 2))
    }

    pub fn inner(&self) -> rust_decimal::Decimal {
        self.0
    }

    pub fn is_zero(&self) -> bool {
        self.0 == rust_decimal::Decimal::ZERO
    }
}

impl From<rust_decimal::Decimal> for Amount {
    fn from(item: rust_decimal::Decimal) -> Self {
        Self(item)
    }
}

impl Dummy<Faker> for Amount {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        Self::new(Faker.fake_with_rng(rng))
    }
}

scalar!(Amount, "Decimal");
