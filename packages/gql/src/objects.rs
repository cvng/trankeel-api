use crate::enums::FileStatus;
use crate::enums::FileType;
use crate::enums::LeaseFurnishedDuration;
use crate::enums::LeaseRentPeriodicity;
use crate::enums::LeaseRentReferenceIrl;
use crate::enums::LeaseStatus;
use crate::enums::LeaseType;
use crate::enums::LegalEntityType;
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
use crate::enums::RentStatus;
use crate::enums::SubscriptionStatus;
use crate::enums::TenantStatus;
use crate::enums::TransactionType;
use crate::enums::UserRole;
use crate::scalars::AuthId;
use crate::scalars::DateTime;
use crate::scalars::Decimal;
use crate::scalars::Email;
use crate::scalars::PhoneNumber;
use crate::unions::Identity;
use crate::wip;
use async_graphql::Result;
use async_graphql::ID;
use async_graphql::*;
use piteo_core::auth;
use piteo_core::DbPool;
use piteo_core::Name;
use std::convert::TryInto;

// # Objects. https://async-graphql.github.io/async-graphql/en/define_simple_object.html

#[derive(async_graphql::SimpleObject)]
#[graphql(complex)]
pub struct Account {
    plan_id: Option<ID>,
    status: Option<SubscriptionStatus>,
    stripe_customer_id: Option<String>,
    stripe_subscription_id: Option<String>,
    trial_end: Option<DateTime>,
    owner_id: Option<String>,
    id: ID,
}

#[async_graphql::ComplexObject]
impl Account {
    async fn plan(&self, _ctx: &Context<'_>) -> Result<Option<Plan>> {
        Ok(None)
    }
}

impl From<piteo_core::Account> for Account {
    fn from(item: piteo_core::Account) -> Self {
        Self {
            plan_id: item.plan_id.map(ID::from),
            status: item.status.map(Into::into),
            stripe_customer_id: item.stripe_customer_id,
            stripe_subscription_id: item.stripe_subscription_id,
            trial_end: item.trial_end.map(DateTime::from),
            owner_id: Some(item.owner_id),
            id: item.id.into(),
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Address {
    city: String,
    country: Option<String>,
    line1: String,
    line2: Option<String>,
    postal_code: String,
    inline: String,
}

impl From<piteo_core::Address> for Address {
    fn from(item: piteo_core::Address) -> Self {
        Self {
            inline: item.inline(),
            city: item.city.unwrap_or_default(),
            country: item.country,
            line1: item.line1.unwrap_or_default(),
            line2: item.line2,
            postal_code: item.postal_code.unwrap_or_default(),
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Company {
    address: Option<Address>,
    display_name: String,
    email: Email,
    id: ID,
    legal_entity: String,
    legal_entity_identifier: Option<String>,
    legal_entity_type: Option<LegalEntityType>,
    legal_entity_type_other: Option<String>,
    phone_number: Option<PhoneNumber>,
}

#[derive(async_graphql::SimpleObject)]
pub struct Feature {
    available: bool,
    title: String,
    key: Option<String>,
}

#[derive(async_graphql::SimpleObject)]
pub struct File {
    created_at: DateTime,
    download_url: Option<String>,
    external_id: Option<String>,
    filename: Option<String>,
    preview_url: Option<String>,
    status: Option<FileStatus>,
    r#type: FileType,
    updated_at: Option<DateTime>,
    id: ID,
}

#[derive(async_graphql::SimpleObject)]
pub struct Invoice {
    id: ID,
    number: u32,
    amount_paid: Decimal,
    invoice_pdf: String,
    period_end: DateTime,
    status: String,
    plan_code: PlanCode,
}

#[derive(async_graphql::SimpleObject)]
pub struct Lease {
    account_id: ID,
    deposit_amount: Option<Decimal>,
    effect_date: DateTime,
    signature_date: Option<DateTime>,
    rent_amount: Decimal,
    rent_charges_amount: Option<Decimal>,
    rent_full_amount: Decimal,
    r#type: LeaseType,
    lease_id: Option<ID>,
    property_id: ID,
    id: ID,
    data: Option<LeaseFurnishedData>,
    expired_at: Option<DateTime>,
    renew_date: Option<DateTime>,
    status: LeaseStatus,
    //
    lease: Option<File>,
    rents: Option<Vec<Rent>>,
    tenants: Vec<Tenant>,
    account: Option<Account>,
    property: Option<Property>,
}

impl From<piteo_core::Lease> for Lease {
    fn from(item: piteo_core::Lease) -> Self {
        Self {
            status: item.status().into(),
            account_id: item.account_id.into(),
            deposit_amount: item.deposit_amount.map(Decimal::from),
            effect_date: item.effect_date.into(),
            signature_date: item.signature_date.map(DateTime::from),
            rent_amount: item.rent_amount.into(),
            rent_charges_amount: item.rent_charges_amount.map(Decimal::from),
            rent_full_amount: item.rent_full_amount().into(),
            r#type: item.r#type.into(),
            lease_id: item.lease_id.map(ID::from),
            property_id: item.property_id.into(),
            id: item.id.into(),
            data: item.data.map(LeaseFurnishedData::from),
            expired_at: item.expired_at.map(DateTime::from),
            renew_date: item.renew_date.map(DateTime::from),
            rents: Some(Vec::new()),
            tenants: Vec::new(),
            lease: None,
            account: None,
            property: None,
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct LeaseFurnishedData {
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
    rent_irl_revision_date: Option<String>,
    rent_maj_decree_increased_amount: Option<Decimal>,
    rent_maj_decree_reference_amount: Option<Decimal>,
    rent_majoration_decree: Option<bool>,
    rent_max_evolution_relocation: Option<bool>,
    rent_payment_date: Option<DateTime>,
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
    tenant_last_rent_received_date: Option<String>,
    tenant_last_rent_revision_date: Option<String>,
    works_decence_since_last_rental: Option<String>,
    works_rent_decrease_tenant: Option<String>,
    works_rent_increase_lender: Option<String>,
}

impl From<piteo_core::LeaseData> for LeaseFurnishedData {
    fn from(item: piteo_core::LeaseData) -> Self {
        Self {
            charges_recuperation_mode: item.charges_recuperation_mode.map(Into::into),
            charges_revision_method: item.charges_revision_method,
            colocation_insurance_lender: item.colocation_insurance_lender,
            colocation_insurance_monthly_amount: item
                .colocation_insurance_monthly_amount
                .map(Into::into),
            colocation_insurance_total_amount: item
                .colocation_insurance_total_amount
                .map(Into::into),
            duration: item.duration.map(Into::into),
            lender_fee_cap: item.lender_fee_cap.map(Into::into),
            lender_fee_cap_other: item.lender_fee_cap_other.map(Into::into),
            lender_fee_cap_prestations: item.lender_fee_cap_prestations.map(Into::into),
            other_conditions: item.other_conditions.map(Into::into),
            rent_complement: item.rent_complement.map(Into::into),
            rent_complement_property_justification: item
                .rent_complement_property_justification
                .map(Into::into),
            rent_first_amount: item.rent_first_amount.map(Into::into),
            rent_irl: item.rent_irl.map(Into::into),
            rent_irl_revision_date: item.rent_irl_revision_date.map(Into::into),
            rent_maj_decree_increased_amount: item.rent_maj_decree_increased_amount.map(Into::into),
            rent_maj_decree_reference_amount: item.rent_maj_decree_reference_amount.map(Into::into),
            rent_majoration_decree: item.rent_majoration_decree.map(Into::into),
            rent_max_evolution_relocation: item.rent_max_evolution_relocation.map(Into::into),
            rent_payment_date: item.rent_payment_date.map(Into::into),
            rent_payment_method: item.rent_payment_method.map(Into::into),
            rent_payment_place: item.rent_payment_place.map(Into::into),
            rent_periodicity: item.rent_periodicity.map(Into::into),
            rent_underestimated_method: item.rent_underestimated_method.map(Into::into),
            rent_underestimated_monthly_variation: item
                .rent_underestimated_monthly_variation
                .map(Into::into),
            resolutary_clause: item.resolutary_clause.map(Into::into),
            solidarity_clause: item.solidarity_clause.map(Into::into),
            tenant_fee_cap_new_rental: item.tenant_fee_cap_new_rental.map(Into::into),
            tenant_fee_cap_prestations: item.tenant_fee_cap_prestations.map(Into::into),
            tenant_fee_cap_report_by_meter: item.tenant_fee_cap_report_by_meter.map(Into::into),
            tenant_fee_cap_report_prestations: item
                .tenant_fee_cap_report_prestations
                .map(Into::into),
            tenant_last_rent_amount: item.tenant_last_rent_amount.map(Into::into),
            tenant_last_rent_received_date: item.tenant_last_rent_received_date.map(Into::into),
            tenant_last_rent_revision_date: item.tenant_last_rent_revision_date.map(Into::into),
            works_decence_since_last_rental: item.works_decence_since_last_rental.map(Into::into),
            works_rent_decrease_tenant: item.works_rent_decrease_tenant.map(Into::into),
            works_rent_increase_lender: item.works_rent_increase_lender.map(Into::into),
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Lender {
    id: ID,
    account_id: Option<ID>,
    individual_id: Option<ID>,
    company_id: Option<ID>,
    display_name: String,
    identity: Option<Identity>,
}

impl From<piteo_core::Lender> for Lender {
    fn from(item: piteo_core::Lender) -> Self {
        Self {
            display_name: item.display_name(),
            id: item.id.into(),
            account_id: Some(item.account_id.into()),
            individual_id: item.individual_id.map(ID::from),
            company_id: item.company_id.map(ID::from),
            identity: None,
        }
    }
}

#[derive(async_graphql::SimpleObject)]
#[graphql(complex)]
#[graphql(name = "User")]
pub struct Person {
    auth_id: Option<AuthId>,
    email: Email,
    first_name: Option<String>,
    last_name: Option<String>,
    address: Option<Address>,
    #[graphql(name = "photoURL")]
    photo_url: Option<String>,
    role: Option<UserRole>,
    id: ID,
    phone_number: Option<PhoneNumber>,
    account_id: Option<ID>,
    display_name: String,
    //
    accounts: Option<Vec<Account>>,
}

#[async_graphql::ComplexObject]
impl Person {
    async fn account(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        let conn = ctx.data::<DbPool>()?.get()?;
        let account_id = self.account_id.clone().ok_or_else(wip)?.try_into()?;

        Ok(Some(
            auth::find_by_id(&conn, account_id).map(Account::from)?,
        ))
    }
}

impl From<piteo_core::Person> for Person {
    fn from(item: piteo_core::Person) -> Self {
        Self {
            display_name: item.display_name(),
            auth_id: Some(item.auth_id.into()),
            email: item.email.into(),
            first_name: item.first_name,
            last_name: item.last_name,
            address: item.address.map(Address::from),
            photo_url: item.photo_url,
            role: item.role.map(Into::into),
            id: item.id.into(),
            phone_number: item.phone_number.map(Into::into),
            account_id: Some(item.account_id.unwrap_or_default().into()),
            accounts: None,
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Plan {
    code: PlanCode,
    price: Option<Decimal>,
    subtitle: Option<String>,
    title: Option<String>,
    id: ID,
    features: Vec<Feature>,
}

#[derive(async_graphql::SimpleObject)]
pub struct Property {
    account: Option<Account>,
    account_id: Option<ID>,
    address: Address,
    build_period: Option<PropertyBuildPeriodType>,
    building_legal_status: Option<PropertyBuildingLegalStatus>,
    common_spaces: Option<String>,
    energy_class: Option<PropertyEnergyClass>,
    equipments: Option<String>,
    gas_emission: Option<PropertyGasEmission>,
    heating_method: Option<PropertyUsageType>,
    housing_type: Option<PropertyUsageType>,
    name: String,
    note: Option<String>,
    ntic_equipments: Option<String>,
    other_spaces: Option<String>,
    tax: Option<f64>,
    room_count: PropertyRoomType,
    status: Option<PropertyStatus>,
    surface: i32,
    tenant_private_spaces: Option<String>,
    usage_type: Option<PropertyHabitationUsageType>,
    water_heating_method: Option<PropertyUsageType>,
    id: ID,
    lender_id: Option<ID>,
    //
    expected_rents: Option<Decimal>,
    collected_rents: Option<Decimal>,
    lender: Option<Lender>,
    leases: Option<Vec<Lease>>,
}

impl From<piteo_core::Property> for Property {
    fn from(item: piteo_core::Property) -> Self {
        Self {
            account_id: item.account_id.map(ID::from),
            address: item.address.into(),
            build_period: item.build_period.map(Into::into),
            building_legal_status: item.building_legal_status.map(Into::into),
            common_spaces: item.common_spaces,
            energy_class: item.energy_class.map(Into::into),
            equipments: item.equipments,
            gas_emission: item.gas_emission.map(Into::into),
            heating_method: item.heating_method.map(Into::into),
            housing_type: item.housing_type.map(Into::into),
            name: item.name,
            note: item.note,
            ntic_equipments: item.ntic_equipments,
            other_spaces: item.other_spaces,
            tax: item.tax,
            room_count: item.room_count.into(),
            status: item.status.map(Into::into),
            surface: item.surface,
            tenant_private_spaces: item.tenant_private_spaces,
            usage_type: item.usage_type.map(Into::into),
            water_heating_method: item.water_heating_method.map(Into::into),
            id: item.id.into(),
            lender_id: Some(item.lender_id.into()),
            account: None,
            expected_rents: None,
            collected_rents: None,
            lender: None,
            leases: Some(Vec::new()),
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Rent {
    id: ID,
    period_end: DateTime,
    period_start: DateTime,
    amount: Decimal,
    charges_amount: Option<Decimal>,
    full_amount: Decimal,
    status: RentStatus,
    lease_id: ID,
    receipt_id: Option<ID>,
    transaction_id: Option<ID>,
    notice_id: Option<ID>,
    //
    delay: Option<i32>,
    transactions: Option<Vec<Transaction>>,
}

impl From<piteo_core::Rent> for Rent {
    fn from(item: piteo_core::Rent) -> Self {
        Self {
            id: item.id.into(),
            period_end: item.period_end.into(),
            period_start: item.period_start.into(),
            amount: item.amount.into(),
            charges_amount: item.charges_amount.map(Decimal::from),
            full_amount: item.full_amount.into(),
            status: item.status.into(),
            lease_id: item.lease_id.into(),
            receipt_id: item.receipt_id.map(ID::from),
            transaction_id: item.transaction_id.map(ID::from),
            notice_id: item.notice_id.map(ID::from),
            delay: None,
            transactions: None,
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Summary {
    since: DateTime,
    until: DateTime,
    //
    amount_expected: Decimal,
    amount_received: Decimal,
    amount_settled: Decimal,
    amount_partial: Decimal,
    amount_pending: Decimal,
    //
    n_expected: u32,
    n_received: u32,
    n_settled: u32,
    n_partial: u32,
    n_pending: u32,
    //
    ratio_expected: f64,
    ratio_received: f64,
    ratio_settled: f64,
    ratio_partial: f64,
    ratio_pending: f64,
    //
    variation_expected: f64,
    variation_received: f64,
    variation_settled: f64,
    variation_partial: f64,
    variation_pending: f64,
    //
    payment_rate: f64,
    occupation_rate: f64,
}

impl From<piteo_core::Summary> for Summary {
    fn from(item: piteo_core::Summary) -> Self {
        Summary {
            since: item.since.into(),
            until: item.until.into(),
            amount_expected: item.amount_expected.into(),
            amount_received: item.amount_received.into(),
            amount_settled: item.amount_settled.into(),
            amount_partial: item.amount_partial.into(),
            amount_pending: item.amount_pending.into(),
            n_expected: item.n_expected,
            n_received: item.n_received,
            n_settled: item.n_settled,
            n_partial: item.n_partial,
            n_pending: item.n_pending,
            ratio_expected: item.ratio_expected,
            ratio_received: item.ratio_received,
            ratio_settled: item.ratio_settled,
            ratio_partial: item.ratio_partial,
            ratio_pending: item.ratio_pending,
            variation_expected: item.variation_expected,
            variation_received: item.variation_received,
            variation_settled: item.variation_settled,
            variation_partial: item.variation_partial,
            variation_pending: item.variation_pending,
            payment_rate: item.payment_rate,
            occupation_rate: item.occupation_rate,
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Task {
    id: ID,
    status: String,
    progress: u32,
}

#[derive(async_graphql::SimpleObject)]
pub struct Tenant {
    account_id: ID,
    apl: Option<bool>,
    auth_id: Option<AuthId>,
    birthdate: DateTime,
    birthplace: Option<String>,
    email: String,
    first_name: String,
    last_name: String,
    display_name: String,
    note: Option<String>,
    phone_number: Option<String>,
    role: Option<String>,
    id: ID,
    lease_id: Option<ID>,
    visale_id: Option<String>,
    //
    account: Option<Account>,
    property: Option<Property>,
    status: Option<TenantStatus>,
    full_name: String,
    short_name: Option<String>,
    last_transaction: Option<Transaction>,
    property_name: Option<String>,
    rent_payed_this_year: Option<String>,
    unpaid_rent_amount: Option<u32>,
    files: Option<Vec<File>>,
    lease: Option<Lease>,
}

impl From<piteo_core::Tenant> for Tenant {
    fn from(item: piteo_core::Tenant) -> Self {
        Self {
            display_name: item.display_name(),
            full_name: item.full_name(),
            account_id: item.account_id.into(),
            apl: Some(item.apl),
            auth_id: item.auth_id.map(AuthId::from),
            birthdate: item.birthdate.into(),
            birthplace: item.birthplace,
            email: item.email,
            first_name: item.first_name,
            last_name: item.last_name,
            note: item.note,
            phone_number: item.phone_number,
            role: item.role,
            id: item.id.into(),
            lease_id: item.lease_id.map(ID::from),
            visale_id: item.visale_id,
            account: None,
            property: None,
            status: Some(TenantStatus::New),
            short_name: None,
            last_transaction: None,
            property_name: None,
            rent_payed_this_year: None,
            unpaid_rent_amount: None,
            files: None,
            lease: None,
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Transaction {
    id: ID,
    date: DateTime,
    amount: Decimal,
    account_id: ID,
    r#type: TransactionType,
    //
    lease: Option<Lease>,
}
