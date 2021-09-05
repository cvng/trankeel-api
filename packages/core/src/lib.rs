mod testing;

pub mod auth;
pub mod billing;
pub mod companies;
pub mod database;
pub mod error;
pub mod files;
pub mod imports;
pub mod leases;
pub mod mailing;
pub mod owners;
pub mod payment;
pub mod properties;
pub mod reports;
pub mod tenants;

pub use piteo_data::schema;
pub use piteo_data::Account;
pub use piteo_data::AccountData;
pub use piteo_data::AccountId;
pub use piteo_data::AccountStatus;
pub use piteo_data::Address;
pub use piteo_data::Amount;
pub use piteo_data::AuthId;
pub use piteo_data::Company;
pub use piteo_data::DateTime;
pub use piteo_data::Email;
pub use piteo_data::FileStatus;
pub use piteo_data::FileType;
pub use piteo_data::ImportSource;
pub use piteo_data::Lease;
pub use piteo_data::LeaseFurnishedData;
pub use piteo_data::LeaseFurnishedDuration;
pub use piteo_data::LeaseNakedDuration;
pub use piteo_data::LeaseRentPeriodicity;
pub use piteo_data::LeaseRentReferenceIrl;
pub use piteo_data::LeaseStatus;
pub use piteo_data::LeaseType;
pub use piteo_data::LegalEntity;
pub use piteo_data::LegalEntityType;
pub use piteo_data::Lender;
pub use piteo_data::LenderData;
pub use piteo_data::LenderId;
pub use piteo_data::LenderIdentity;
pub use piteo_data::Name;
pub use piteo_data::Person;
pub use piteo_data::PersonData;
pub use piteo_data::PersonId;
pub use piteo_data::Plan;
pub use piteo_data::PlanCode;
pub use piteo_data::Property;
pub use piteo_data::PropertyBuildPeriodType;
pub use piteo_data::PropertyBuildingLegalStatus;
pub use piteo_data::PropertyData;
pub use piteo_data::PropertyEnergyClass;
pub use piteo_data::PropertyGasEmission;
pub use piteo_data::PropertyHabitationUsageType;
pub use piteo_data::PropertyId;
pub use piteo_data::PropertyRoomType;
pub use piteo_data::PropertyStatus;
pub use piteo_data::PropertyUsageType;
pub use piteo_data::Rent;
pub use piteo_data::RentChargesRecuperationMode;
pub use piteo_data::RentPaymentMethod;
pub use piteo_data::RentStatus;
pub use piteo_data::Subscription;
pub use piteo_data::Summary;
pub use piteo_data::Tenant;
pub use piteo_data::TenantData;
pub use piteo_data::TenantId;
pub use piteo_data::TenantStatus;
pub use piteo_data::TransactionType;
pub use piteo_data::UserRole;

// TODO: Move ops to lib then remove this.
type Conn = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
