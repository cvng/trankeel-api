use async_graphql::Enum;
use serde::Deserialize;
use serde::Serialize;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LeaseFurnishedDuration {
    NineMonths,
    OneYear,
}

/// https://www.service-public.fr/particuliers/vosdroits/F13723
#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LeaseRentReferenceIrl {
    AprilFirstSemesterY2021,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LeaseRentPeriodicity {
    Annualy,
    Monthly,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RentChargesRecuperationMode {
    Package,
    Periodic,
    Reel,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RentPaymentMethod {
    After,
    Before,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LeaseNakedDuration {
    ThreeYears,
    SixYears,
}
