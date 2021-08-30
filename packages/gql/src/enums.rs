// # Enums. https://async-graphql.github.io/async-graphql/en/define_enum.html

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
