// # Enums. https://async-graphql.github.io/async-graphql/en/define_enum.html

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum UserRole {
    Admin,
    User,
    Viewer,
}

impl From<String> for UserRole {
    fn from(item: String) -> Self {
        match item.as_str() {
            "ADMIN" => UserRole::Admin,
            "USER" => UserRole::User,
            "VIEWER" => UserRole::Viewer,
            _ => unimplemented!(),
        }
    }
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum PlanCode {
    Solo,
}

/// https://stripe.com/docs/billing/subscriptions/overview#subscription-states
#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum SubscriptionStatus {
    Active,
    Canceled,
    Incomplete,
    IncompleteExpired,
    PastDue,
    Trialing,
    Unpaid,
}

impl From<String> for SubscriptionStatus {
    fn from(item: String) -> Self {
        match item.as_str() {
            "ACTIVE" => Self::Active,
            "CANCELED" => Self::Canceled,
            "INCOMPLETE" => Self::Incomplete,
            "INCOMPLETE_EXPIRED" => Self::IncompleteExpired,
            "PAST_DUE" => Self::PastDue,
            "TRIALING" => Self::Trialing,
            "UNPAID" => Self::Unpaid,
            _ => unimplemented!(),
        }
    }
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
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

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
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

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
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

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
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

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
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

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
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

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
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

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
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

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum LegalEntityType {
    Eurl,
    Other,
    Sa,
    Sarl,
    Sas,
    Sasu,
    Sci,
    Scp,
    Snc,
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum LeaseNakedDuration {
    ThreeYears,
    SixYears,
}

/// https://www.service-public.fr/particuliers/vosdroits/F13723
#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum LeaseRentReferenceIrl {
    AprilFirstSemesterY2021,
}

impl From<piteo_core::LeaseRentReferenceIrl> for LeaseRentReferenceIrl {
    fn from(item: piteo_core::LeaseRentReferenceIrl) -> Self {
        match item {
            piteo_core::LeaseRentReferenceIrl::AprilFirstSemesterY2021 => {
                Self::AprilFirstSemesterY2021
            }
        }
    }
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum LeaseRentPeriodicity {
    Annualy,
    Monthly,
}

impl From<piteo_core::LeaseRentPeriodicity> for LeaseRentPeriodicity {
    fn from(item: piteo_core::LeaseRentPeriodicity) -> Self {
        match item {
            piteo_core::LeaseRentPeriodicity::Annualy => Self::Annualy,
            piteo_core::LeaseRentPeriodicity::Monthly => Self::Monthly,
        }
    }
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum RentPaymentMethod {
    After,
    Before,
}

impl From<piteo_core::RentPaymentMethod> for RentPaymentMethod {
    fn from(item: piteo_core::RentPaymentMethod) -> Self {
        match item {
            piteo_core::RentPaymentMethod::After => Self::After,
            piteo_core::RentPaymentMethod::Before => Self::Before,
        }
    }
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum RentChargesRecuperationMode {
    Package,
    Periodic,
    Reel,
}

impl From<piteo_core::RentChargesRecuperationMode> for RentChargesRecuperationMode {
    fn from(item: piteo_core::RentChargesRecuperationMode) -> Self {
        match item {
            piteo_core::RentChargesRecuperationMode::Package => Self::Package,
            piteo_core::RentChargesRecuperationMode::Periodic => Self::Periodic,
            piteo_core::RentChargesRecuperationMode::Reel => Self::Reel,
        }
    }
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum TransactionType {
    InsuranceHab,
    InsurancePno,
    Invoice,
    LoanInterest,
    LoanPayment,
    Other,
    Rent,
}

impl From<String> for TransactionType {
    fn from(item: String) -> Self {
        match item.as_str() {
            "INSURANCE_HAB" => Self::InsuranceHab,
            "INSURANCE_PNO" => Self::InsurancePno,
            "INVOICE" => Self::Invoice,
            "LOAN_INTEREST" => Self::LoanInterest,
            "LOAN_PAYMENT" => Self::LoanPayment,
            "OTHER" => Self::Other,
            "RENT" => Self::Rent,
            _ => unimplemented!(),
        }
    }
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum TaskStatus {
    Completed,
    Failed,
    Pending,
    Running,
}

/// https://www.pdfmonkey.io/fr/doc/api/generer-un-document
#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum FileStatus {
    Draft,
    Failure,
    Generating,
    Pending,
    Success,
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum FileType {
    PaymentNotice,
    LeaseDocument,
    RentReceipt,
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum ImportSource {
    Rentila,
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum LeaseFurnishedDuration {
    NineMonths,
    OneYear,
}

impl From<piteo_core::LeaseFurnishedDuration> for LeaseFurnishedDuration {
    fn from(item: piteo_core::LeaseFurnishedDuration) -> Self {
        match item {
            piteo_core::LeaseFurnishedDuration::NineMonths => Self::NineMonths,
            piteo_core::LeaseFurnishedDuration::OneYear => Self::OneYear,
        }
    }
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum LeaseStatus {
    Active,
    Ended,
}

impl From<piteo_core::LeaseStatus> for LeaseStatus {
    fn from(item: piteo_core::LeaseStatus) -> Self {
        match item {
            piteo_core::LeaseStatus::Active => Self::Active,
            piteo_core::LeaseStatus::Ended => Self::Ended,
        }
    }
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum LeaseType {
    Furnished,
    Naked,
}

impl From<piteo_core::LeaseType> for LeaseType {
    fn from(item: piteo_core::LeaseType) -> Self {
        match item {
            piteo_core::LeaseType::Furnished => Self::Furnished,
            piteo_core::LeaseType::Naked => Self::Naked,
        }
    }
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum RentStatus {
    Partial,
    Pending,
    Settled,
}

impl From<piteo_core::RentStatus> for RentStatus {
    fn from(item: piteo_core::RentStatus) -> Self {
        match item {
            piteo_core::RentStatus::Partial => Self::Partial,
            piteo_core::RentStatus::Pending => Self::Pending,
            piteo_core::RentStatus::Settled => Self::Settled,
        }
    }
}

#[derive(async_graphql::Enum, Copy, Clone, Eq, PartialEq)]
pub enum TenantStatus {
    Gone,
    Late,
    New,
    Uptodate,
}

impl From<piteo_core::TenantStatus> for TenantStatus {
    fn from(item: piteo_core::TenantStatus) -> Self {
        match item {
            piteo_core::TenantStatus::Gone => Self::Gone,
            piteo_core::TenantStatus::Late => Self::Late,
            piteo_core::TenantStatus::New => Self::New,
            piteo_core::TenantStatus::Uptodate => Self::Uptodate,
        }
    }
}
