use std::fmt;
use std::fmt::Display;
use std::ops::Add;

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize, DieselNewType)]
pub struct Amount(rust_decimal::Decimal);

impl Amount {
    pub fn new(num: i64, scale: u32) -> Self {
        Self(rust_decimal::Decimal::new(num, scale))
    }

    pub fn inner(&self) -> rust_decimal::Decimal {
        self.0
    }
}

impl Add for Amount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.add(rhs.0))
    }
}

impl Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl From<rust_decimal::Decimal> for Amount {
    fn from(item: rust_decimal::Decimal) -> Self {
        Self(item)
    }
}

scalar!(Amount, "Decimal");
