use async_graphql::Enum;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PropertyRoomType {
    Other,
    T1,
    T2,
    T3,
    T4,
    T5,
    T6,
}

impl From<String> for PropertyRoomType {
    fn from(item: String) -> Self {
        match item.as_str() {
            "OTHER" => Self::Other,
            "T1" => Self::T1,
            "T2" => Self::T2,
            "T3" => Self::T3,
            "T4" => Self::T4,
            "T5" => Self::T5,
            "T6" => Self::T6,
            _ => unimplemented!(),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PropertyStatus {
    ForSale,
    Inactive,
    Rented,
    UnderConstruction,
    Unrented,
}

impl From<String> for PropertyStatus {
    fn from(item: String) -> Self {
        match item.as_str() {
            "FOR_SALE" => Self::ForSale,
            "INACTIVE" => Self::Inactive,
            "RENTED" => Self::Rented,
            "UNDER_CONSTRUCTION" => Self::UnderConstruction,
            "UNRENTED" => Self::Unrented,
            _ => unimplemented!(),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PropertyBuildPeriodType {
    BeforeY1949,
    FromY1949Y1974,
    FromY1975Y1989,
    FromY1990Y2005,
    FromY2005,
}

impl From<String> for PropertyBuildPeriodType {
    fn from(item: String) -> Self {
        match item.as_str() {
            "BEFORE_Y1949" => Self::BeforeY1949,
            "FROM_Y1949_Y1974" => Self::FromY1949Y1974,
            "FROM_Y1975_Y1989" => Self::FromY1975Y1989,
            "FROM_Y1990_Y2005" => Self::FromY1990Y2005,
            "FROM_Y2005" => Self::FromY2005,
            _ => unimplemented!(),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PropertyEnergyClass {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl From<String> for PropertyEnergyClass {
    fn from(item: String) -> Self {
        match item.as_str() {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            "E" => Self::E,
            "F" => Self::F,
            _ => unimplemented!(),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PropertyGasEmission {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl From<String> for PropertyGasEmission {
    fn from(item: String) -> Self {
        match item.as_str() {
            "A" => Self::A,
            "B" => Self::B,
            "C" => Self::C,
            "D" => Self::D,
            "E" => Self::E,
            "F" => Self::F,
            _ => unimplemented!(),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PropertyBuildingLegalStatus {
    Copro,
    Mono,
}

impl From<String> for PropertyBuildingLegalStatus {
    fn from(item: String) -> Self {
        match item.as_str() {
            "MONO" => Self::Mono,
            "COPRO" => Self::Copro,
            _ => unimplemented!(),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PropertyHabitationUsageType {
    Habitation,
    Mixte,
}

impl From<String> for PropertyHabitationUsageType {
    fn from(item: String) -> Self {
        match item.as_str() {
            "HABITATION" => Self::Habitation,
            "MIXTE" => Self::Mixte,
            _ => unimplemented!(),
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PropertyUsageType {
    Collective,
    Individual,
}

impl From<String> for PropertyUsageType {
    fn from(item: String) -> Self {
        match item.as_str() {
            "COLLECTIVE" => Self::Collective,
            "INDIVIDUAL" => Self::Individual,
            _ => unimplemented!(),
        }
    }
}
