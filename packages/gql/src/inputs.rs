use crate::enums::FileType;
use crate::enums::ImportSource;
use crate::enums::LeaseFurnishedDuration;
use crate::enums::LeaseRentPeriodicity;
use crate::enums::LeaseRentReferenceIrl;
use crate::enums::PlanCode;
use crate::enums::PropertyBuildPeriodType;
use crate::enums::PropertyBuildingLegalStatus;
use crate::enums::PropertyEnergyClass;
use crate::enums::PropertyGasEmission;
use crate::enums::PropertyHabitationUsageType;
use crate::enums::PropertyRoomType;
use crate::enums::PropertyStatus;
use crate::enums::PropertyUsageType;
use crate::enums::RentChargesRecuperationMode;
use crate::enums::RentPaymentMethod;
use crate::enums::TransactionType;
use crate::scalars::AuthId;
use crate::scalars::Date;
use crate::scalars::DateTime;
use crate::scalars::Decimal;
use crate::scalars::Email;
use crate::scalars::PhoneNumber;
use async_graphql::ID;

#[derive(async_graphql::InputObject)]
pub struct UserWithAccountInput {
    address: Option<AddressInput>,
    auth_id: AuthId,
    email: Email,
    first_name: String,
    last_name: String,
    skip_create_customer: Option<bool>,
}

#[derive(async_graphql::InputObject)]
pub struct UserInput {
    address: AddressInput,
    email: Email,
    first_name: String,
    last_name: String,
}

#[derive(async_graphql::InputObject)]
pub struct UserUpdateInput {
    address: AddressInput,
    first_name: String,
    last_name: String,
}

#[derive(async_graphql::InputObject)]
pub struct AccountActivatePlanInput {
    id: ID,
    name: String,
    plan_code: PlanCode,
}

#[derive(async_graphql::InputObject)]
pub struct AccountUpdateInput {
    id: ID,
    payment_method_id: ID,
}

#[derive(async_graphql::InputObject)]
pub struct AddressInput {
    city: String,
    country: Option<String>,
    line1: String,
    line2: Option<String>,
    postal_code: String,
}

#[derive(async_graphql::InputObject)]
pub struct PropertyInput {
    address: AddressInput,
    build_period: PropertyBuildPeriodType,
    building_legal_status: PropertyBuildingLegalStatus,
    common_spaces: Option<String>,
    energy_class: Option<PropertyEnergyClass>,
    equipments: Option<String>,
    gas_emission: Option<PropertyGasEmission>,
    heating_method: PropertyUsageType,
    housing_type: PropertyUsageType,
    lender_id: ID,
    name: String,
    note: Option<String>,
    ntic_equipments: Option<String>,
    other_spaces: Option<String>,
    room_count: PropertyRoomType,
    status: Option<PropertyStatus>,
    surface: f64,
    tax: Option<Decimal>,
    tenant_private_spaces: Option<String>,
    usage_type: PropertyHabitationUsageType,
    water_heating_method: PropertyUsageType,
}

#[derive(async_graphql::InputObject)]
pub struct PropertyUpdateInput {
    address: Option<AddressInput>,
    build_period: Option<PropertyBuildPeriodType>,
    building_legal_status: Option<PropertyBuildingLegalStatus>,
    common_spaces: Option<String>,
    energy_class: Option<PropertyEnergyClass>,
    equipments: Option<String>,
    gas_emission: Option<PropertyGasEmission>,
    heating_method: Option<PropertyUsageType>,
    housing_type: Option<PropertyUsageType>,
    id: ID,
    name: Option<String>,
    note: Option<String>,
    ntic_equipments: Option<String>,
    other_spaces: Option<String>,
    room_count: Option<PropertyRoomType>,
    status: Option<PropertyStatus>,
    surface: Option<f64>,
    tax: Option<Decimal>,
    tenant_private_spaces: Option<String>,
    usage_type: Option<PropertyHabitationUsageType>,
    water_heating_method: Option<PropertyUsageType>,
}

#[derive(async_graphql::InputObject)]
pub struct TenantInput {
    apl: Option<bool>,
    birthdate: Date,
    birthplace: Option<String>,
    email: Email,
    first_name: String,
    last_name: String,
    note: Option<String>,
    phone_number: Option<PhoneNumber>,
    visale_id: Option<String>,
}

impl From<TenantInput> for piteo_core::tenants::TenantInput {
    fn from(item: TenantInput) -> Self {
        Self {
            apl: item.apl,
            birthdate: item.birthdate.into(),
            birthplace: item.birthplace,
            email: item.email.into(),
            first_name: item.first_name,
            last_name: item.last_name,
            note: item.note,
            phone_number: item.phone_number.map(Into::into),
            visale_id: item.visale_id,
        }
    }
}

#[derive(async_graphql::InputObject)]
pub struct TenantUpdateInput {
    apl: Option<bool>,
    birthdate: Option<Date>,
    birthplace: Option<String>,
    email: Option<Email>,
    id: ID,
    first_name: Option<String>,
    last_name: Option<String>,
    note: Option<String>,
    phone_number: Option<PhoneNumber>,
    visale_id: Option<String>,
}

#[derive(async_graphql::InputObject)]
pub struct CompanyInput {
    address: AddressInput,
    email: Email,
    legal_entity: String,
}

#[derive(async_graphql::InputObject)]
pub struct CompanyUpdateInput {
    address: Option<AddressInput>,
}

#[derive(async_graphql::InputObject)]
pub struct LenderIndividualInput {
    individual: UserInput,
}

#[derive(async_graphql::InputObject)]
pub struct LenderCompanyInput {
    company: CompanyInput,
}

#[derive(async_graphql::InputObject)]
pub struct LenderIndividualUpdateInput {
    id: ID,
    individual: UserUpdateInput,
}

#[derive(async_graphql::InputObject)]
pub struct LenderCompanyUpdateInput {
    company: CompanyUpdateInput,
    id: ID,
}

#[derive(async_graphql::InputObject)]
pub struct LeaseFurnishedInput {
    data: Option<LeaseFurnishedDataInput>,
    deposit_amount: Option<Decimal>,
    effect_date: Date,
    renew_date: Option<Date>,
    file: Option<FileInput>,
    property_id: ID,
    rent_amount: Decimal,
    rent_charges_amount: Option<Decimal>,
    signature_date: Option<Date>,
    tenant_ids: Vec<ID>,
    // r#type: LeaseType,
}

#[derive(async_graphql::InputObject)]
pub struct LeaseFurnishedUpdateInput {
    data: Option<LeaseFurnishedDataInput>,
    file: Option<FileInput>,
    id: ID,
}

#[derive(async_graphql::InputObject)]
pub struct DocumentGenerateInput {
    id: ID,
    r#type: FileType,
}

#[derive(async_graphql::InputObject)]
pub struct SendPaymentNoticeInput {
    lease_id: ID,
    date: Option<Date>,
}

#[derive(async_graphql::InputObject)]
pub struct LeaseFurnishedDataInput {
    charges_recuperation_mode: Option<RentChargesRecuperationMode>,
    charges_revision_method: Option<String>,
    colocation_insurance_lender: Option<bool>,
    colocation_insurance_monthly_amount: Option<Decimal>,
    colocation_insurance_total_amount: Option<Decimal>,
    duration: Option<LeaseFurnishedDuration>,
    lender_fee_cap: Option<Decimal>,
    lender_fee_cap_other: Option<String>,
    lender_fee_cap_prestations: Option<Decimal>,
    other_conditions: Option<String>,
    rent_complement: Option<Decimal>,
    rent_complement_property_justification: Option<String>,
    rent_first_amount: Option<Decimal>,
    rent_irl: Option<LeaseRentReferenceIrl>,
    rent_irl_revision_date: Option<Date>,
    rent_maj_decree_increased_amount: Option<Decimal>,
    rent_maj_decree_reference_amount: Option<Decimal>,
    rent_majoration_decree: Option<bool>,
    rent_max_evolution_relocation: Option<bool>,
    rent_payment_date: Option<Date>,
    rent_payment_method: Option<RentPaymentMethod>,
    rent_payment_place: Option<String>,
    rent_periodicity: Option<LeaseRentPeriodicity>,
    rent_underestimated_method: Option<String>,
    rent_underestimated_monthly_variation: Option<Decimal>,
    resolutary_clause: Option<String>,
    solidarity_clause: Option<String>,
    tenant_fee_cap_new_rental: Option<Decimal>,
    tenant_fee_cap_prestations: Option<Decimal>,
    tenant_fee_cap_report_by_meter: Option<Decimal>,
    tenant_fee_cap_report_prestations: Option<Decimal>,
    tenant_last_rent_amount: Option<Decimal>,
    tenant_last_rent_received_date: Option<Date>,
    tenant_last_rent_revision_date: Option<Date>,
    works_decence_since_last_rental: Option<String>,
    works_rent_decrease_tenant: Option<String>,
    works_rent_increase_lender: Option<String>,
}

#[derive(async_graphql::InputObject)]
pub struct RentInput {
    amount: Decimal,
    charges_amount: Option<Decimal>,
    lease_id: ID,
    period_end: Option<DateTime>,
    period_start: Option<DateTime>,
}

#[derive(async_graphql::InputObject)]
pub struct RentReceiptInput {
    lease_id: ID,
    send_mail: Option<bool>,
}

#[derive(async_graphql::InputObject)]
pub struct RentReceiptsInput {
    send_mail: Option<bool>,
}

#[derive(async_graphql::InputObject)]
pub struct TransactionInput {
    amount: Decimal,
    lease_id: ID,
    date: Date,
    r#type: Option<TransactionType>,
}

#[derive(async_graphql::InputObject)]
pub struct FileInput {
    download_url: String,
    r#type: FileType,
}

#[derive(async_graphql::InputObject)]
pub struct ImportInput {
    files: Vec<FileInput>,
    source: ImportSource,
}

#[derive(async_graphql::InputObject)]
pub struct MailSendInput {
    id: ID,
    r#type: FileType,
}
