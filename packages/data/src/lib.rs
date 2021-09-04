#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_derive_newtype;

mod account;
mod address;
mod common;
mod company;
mod file;
mod import;
mod lease;
mod lease_data;
mod lender;
mod locale;
mod person;
mod plan;
mod property;
mod rent;
mod subscription;
mod summary;
mod task;
mod tenant;
mod transaction;

pub mod schema;

pub use crate::account::Account;
pub use crate::account::AccountData;
pub use crate::account::AccountId;
pub use crate::account::AccountStatus;
pub use crate::address::Address;
pub use crate::common::Amount;
pub use crate::common::DateTime;
pub use crate::common::Email;
pub use crate::common::Id;
pub use crate::common::LegalEntity;
pub use crate::common::Name;
pub use crate::common::PhoneNumber;
pub use crate::company::Company;
pub use crate::company::CompanyId;
pub use crate::company::LegalEntityType;
pub use crate::file::File;
pub use crate::file::FileStatus;
pub use crate::file::FileType;
pub use crate::import::ImportSource;
pub use crate::lease::Lease;
pub use crate::lease::LeaseStatus;
pub use crate::lease::LeaseType;
pub use crate::lease_data::LeaseData;
pub use crate::lease_data::LeaseFurnishedData;
pub use crate::lease_data::LeaseFurnishedDuration;
pub use crate::lease_data::LeaseNakedDuration;
pub use crate::lease_data::LeaseRentPeriodicity;
pub use crate::lease_data::LeaseRentReferenceIrl;
pub use crate::lease_data::RentChargesRecuperationMode;
pub use crate::lease_data::RentPaymentMethod;
pub use crate::lender::Lender;
pub use crate::lender::LenderData;
pub use crate::lender::LenderIdentity;
pub use crate::person::AuthId;
pub use crate::person::Person;
pub use crate::person::PersonData;
pub use crate::person::PersonId;
pub use crate::person::UserRole;
pub use crate::plan::Plan;
pub use crate::plan::PlanCode;
pub use crate::plan::PlanId;
pub use crate::property::Property;
pub use crate::property::PropertyBuildPeriodType;
pub use crate::property::PropertyBuildingLegalStatus;
pub use crate::property::PropertyEnergyClass;
pub use crate::property::PropertyGasEmission;
pub use crate::property::PropertyHabitationUsageType;
pub use crate::property::PropertyRoomType;
pub use crate::property::PropertyStatus;
pub use crate::property::PropertyUsageType;
pub use crate::rent::Rent;
pub use crate::rent::RentStatus;
pub use crate::subscription::CustomerId;
pub use crate::subscription::Subscription;
pub use crate::subscription::SubscriptionId;
pub use crate::summary::Summary;
pub use crate::tenant::Tenant;
pub use crate::tenant::TenantData;
pub use crate::tenant::TenantId;
pub use crate::tenant::TenantStatus;
pub use crate::transaction::TransactionType;
