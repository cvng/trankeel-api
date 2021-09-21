use crate::scalars::AuthId;
use crate::scalars::DateTime;
use crate::scalars::Decimal;
use crate::scalars::Email;
use crate::scalars::PhoneNumber;
use crate::unions::Eventable;
use crate::unions::Identity;
use async_graphql::Result;
use async_graphql::ID;
use async_graphql::*;
use piteo::database::Db;
use piteo::db;
use piteo::AccountStatus;
use piteo::DbPool;
use piteo::EventType;
use piteo::FileStatus;
use piteo::FileType;
use piteo::FurnishedLeaseDuration;
use piteo::LeaseRentPeriodicity;
use piteo::LeaseRentReferenceIrl;
use piteo::LeaseStatus;
use piteo::LeaseType;
use piteo::LegalEntityType;
use piteo::Name;
use piteo::PersonRole;
use piteo::PlanCode;
use piteo::PropertyBuildPeriodType;
use piteo::PropertyBuildingLegalStatus;
use piteo::PropertyEnergyClass;
use piteo::PropertyGasEmission;
use piteo::PropertyHabitationUsageType;
use piteo::PropertyRoomType;
use piteo::PropertyStatus;
use piteo::PropertyUsageType;
use piteo::RentChargesRecuperationMode;
use piteo::RentPaymentMethod;
use piteo::RentStatus;
use piteo::TenantStatus;
use piteo::TransactionType;
use std::convert::TryInto;

// # Objects. https://async-graphql.github.io/async-graphql/en/define_complex_object.html

#[derive(async_graphql::SimpleObject)]
pub struct Error {
    message: String,
}

impl From<piteo::error::Error> for Error {
    fn from(item: piteo::error::Error) -> Self {
        Self {
            message: item.to_string(),
        }
    }
}

#[derive(async_graphql::SimpleObject)]
#[graphql(complex)]
pub struct Account {
    plan_id: Option<ID>,
    status: Option<AccountStatus>,
    stripe_customer_id: Option<String>,
    stripe_subscription_id: Option<String>,
    trial_end: Option<DateTime>,
    id: ID,
}

#[async_graphql::ComplexObject]
impl Account {
    async fn plan(&self, _ctx: &Context<'_>) -> Result<Option<Plan>> {
        Ok(None)
    }
}

impl From<piteo::Account> for Account {
    fn from(item: piteo::Account) -> Self {
        Self {
            plan_id: item.plan_id.map(Into::into),
            status: item.status.map(Into::into),
            stripe_customer_id: item.stripe_customer_id,
            stripe_subscription_id: item.stripe_subscription_id,
            trial_end: item.trial_end.map(Into::into),
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

impl From<piteo::Address> for Address {
    fn from(item: piteo::Address) -> Self {
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

impl From<piteo::Company> for Company {
    fn from(item: piteo::Company) -> Self {
        Self {
            display_name: item.display_name(),
            address: item.address.map(Into::into),
            email: item.email.into(),
            id: item.id.into(),
            legal_entity: item.legal_entity,
            legal_entity_identifier: item.legal_entity_identifier,
            legal_entity_type: item.legal_entity_type.map(Into::into),
            legal_entity_type_other: item.legal_entity_type_other,
            phone_number: item.phone_number.map(Into::into),
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Feature {
    available: bool,
    title: String,
    key: Option<String>,
}

#[derive(async_graphql::SimpleObject)]
pub struct File {
    created_at: Option<DateTime>,
    download_url: Option<String>,
    external_id: Option<String>,
    filename: Option<String>,
    preview_url: Option<String>,
    status: Option<FileStatus>,
    r#type: FileType,
    updated_at: Option<DateTime>,
    id: ID,
}

impl From<piteo::File> for File {
    fn from(item: piteo::File) -> Self {
        Self {
            created_at: item.created_at.map(Into::into),
            download_url: item.download_url,
            external_id: item.external_id,
            filename: item.filename,
            preview_url: item.preview_url,
            status: item.status,
            r#type: item.type_,
            updated_at: item.updated_at.map(Into::into),
            id: item.id.into(),
        }
    }
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

pub struct Lease(piteo::Lease);

#[async_graphql::Object]
impl Lease {
    async fn account_id(&self) -> ID {
        self.0.account_id.into()
    }
    async fn deposit_amount(&self) -> Option<Decimal> {
        self.0.deposit_amount.map(Into::into)
    }
    async fn effect_date(&self) -> DateTime {
        self.0.effect_date.into()
    }
    async fn duration(&self) -> FurnishedLeaseDuration {
        self.0.duration
    }
    async fn signature_date(&self) -> Option<DateTime> {
        self.0.signature_date.map(Into::into)
    }
    async fn rent_amount(&self) -> Decimal {
        self.0.rent_amount.into()
    }
    async fn rent_charges_amount(&self) -> Option<Decimal> {
        self.0.rent_charges_amount.map(Into::into)
    }
    async fn rent_full_amount(&self) -> Decimal {
        self.0.rent_full_amount().into()
    }
    async fn r#type(&self) -> LeaseType {
        self.0.type_
    }
    async fn lease_id(&self) -> Option<ID> {
        self.0.lease_id.map(Into::into)
    }
    async fn property_id(&self) -> ID {
        self.0.property_id.into()
    }
    async fn id(&self) -> ID {
        self.0.id.into()
    }
    #[graphql(name = "data")]
    async fn details(&self) -> Option<FurnishedLeaseDetails> {
        self.0.details.clone().map(Into::into)
    }
    async fn expired_at(&self) -> Option<DateTime> {
        self.0.expired_at.map(Into::into)
    }
    async fn renew_date(&self) -> Option<DateTime> {
        self.0.renew_date.map(Into::into)
    }
    async fn status(&self) -> LeaseStatus {
        self.0.status()
    }
    async fn rents(&self, ctx: &Context<'_>) -> Result<Vec<Rent>> {
        let pool = ctx.data::<DbPool>()?;
        Ok(piteo::db(pool.clone())
            .rents()
            .by_lease_id(&self.0.id)?
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>())
    }
    async fn lease(&self) -> Option<File> {
        None
    }
    async fn tenants(&self) -> Vec<Tenant> {
        Vec::new()
    }
    async fn account(&self) -> Option<Account> {
        None
    }
    async fn property(&self) -> Option<Property> {
        None
    }
}

impl From<piteo::Lease> for Lease {
    fn from(item: piteo::Lease) -> Self {
        Self(item)
    }
}

#[derive(async_graphql::SimpleObject)]
#[graphql(name = "LeaseFurnishedData")]
pub struct FurnishedLeaseDetails {
    charges_recuperation_mode: Option<RentChargesRecuperationMode>,
    charges_revision_method: Option<String>,
    colocation_insurance_lender: Option<bool>,
    colocation_insurance_monthly_amount: Option<Decimal>,
    colocation_insurance_total_amount: Option<Decimal>,
    duration: Option<FurnishedLeaseDuration>,
    lender_fee_cap: Option<Decimal>,
    lender_fee_cap_other: Option<String>,
    lender_fee_cap_prestations: Option<Decimal>,
    other_conditions: Option<String>,
    rent_complement: Option<Decimal>,
    rent_complement_property_justification: Option<String>,
    rent_first_amount: Option<Decimal>,
    rent_irl: Option<LeaseRentReferenceIrl>,
    rent_irl_revision_date: Option<DateTime>,
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
    tenant_last_rent_received_date: Option<DateTime>,
    tenant_last_rent_revision_date: Option<DateTime>,
    works_decence_since_last_rental: Option<String>,
    works_rent_decrease_tenant: Option<String>,
    works_rent_increase_lender: Option<String>,
}

impl From<piteo::FurnishedLeaseDetails> for FurnishedLeaseDetails {
    fn from(item: piteo::FurnishedLeaseDetails) -> Self {
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

pub struct Lender(piteo::Lender);

#[async_graphql::Object]
impl Lender {
    async fn id(&self) -> ID {
        self.0.id.into()
    }
    async fn account_id(&self) -> ID {
        self.0.account_id.into()
    }
    async fn individual_id(&self) -> Option<ID> {
        self.0.individual_id.map(Into::into)
    }
    async fn company_id(&self) -> Option<ID> {
        self.0.company_id.map(Into::into)
    }
    async fn display_name(&self, ctx: &Context<'_>) -> Result<String> {
        let db_pool = ctx.data::<DbPool>()?;
        Ok(db(db_pool.clone())
            .lenders()
            .by_id(&self.0.id)?
            .display_name())
    }
    async fn identity(&self, ctx: &Context<'_>) -> Result<Identity> {
        let db_pool = ctx.data::<DbPool>()?;
        Ok(db(db_pool.clone()).lenders().by_id(&self.0.id)?.into())
    }
}

impl From<piteo::Lender> for Lender {
    fn from(item: piteo::Lender) -> Self {
        Self(item)
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
    role: Option<PersonRole>,
    id: ID,
    phone_number: Option<PhoneNumber>,
    account_id: ID,
    display_name: String,
    //
    accounts: Option<Vec<Account>>,
}

#[async_graphql::ComplexObject]
impl Person {
    async fn account(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        let db_pool = ctx.data::<DbPool>()?;
        let account_id = self.account_id.clone().try_into()?;

        Ok(Some(
            db(db_pool.clone())
                .accounts()
                .by_id(&account_id)
                .map(Into::into)?,
        ))
    }
}

impl From<piteo::Person> for Person {
    fn from(item: piteo::Person) -> Self {
        Self {
            display_name: item.display_name(),
            auth_id: Some(item.auth_id.into()),
            email: item.email.into(),
            first_name: item.first_name,
            last_name: item.last_name,
            address: item.address.map(Into::into),
            photo_url: item.photo_url,
            role: item.role.map(Into::into),
            id: item.id.into(),
            phone_number: item.phone_number.map(Into::into),
            account_id: item.account_id.into(),
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

pub struct Property(piteo::Property);

#[async_graphql::Object]
impl Property {
    async fn account(&self) -> Option<Account> {
        None
    }
    async fn account_id(&self) -> ID {
        self.0.account_id.into()
    }
    async fn address(&self) -> Address {
        self.0.address.clone().into()
    }
    async fn build_period(&self) -> Option<PropertyBuildPeriodType> {
        self.0.build_period.map(Into::into)
    }
    async fn building_legal_status(&self) -> Option<PropertyBuildingLegalStatus> {
        self.0.building_legal_status.map(Into::into)
    }
    async fn common_spaces(&self) -> Option<String> {
        self.0.common_spaces.clone()
    }
    async fn energy_class(&self) -> Option<PropertyEnergyClass> {
        self.0.energy_class.map(Into::into)
    }
    async fn equipments(&self) -> Option<String> {
        self.0.equipments.clone()
    }
    async fn gas_emission(&self) -> Option<PropertyGasEmission> {
        self.0.gas_emission.map(Into::into)
    }
    async fn heating_method(&self) -> Option<PropertyUsageType> {
        self.0.heating_method.map(Into::into)
    }
    async fn housing_type(&self) -> Option<PropertyUsageType> {
        self.0.housing_type.map(Into::into)
    }
    async fn name(&self) -> String {
        self.0.name.clone()
    }
    async fn note(&self) -> Option<String> {
        self.0.note.clone()
    }
    async fn ntic_equipments(&self) -> Option<String> {
        self.0.ntic_equipments.clone()
    }
    async fn other_spaces(&self) -> Option<String> {
        self.0.other_spaces.clone()
    }
    async fn tax(&self) -> Option<Decimal> {
        self.0.tax.map(Into::into)
    }
    async fn room_count(&self) -> PropertyRoomType {
        self.0.room_count
    }
    async fn status(&self) -> Option<PropertyStatus> {
        self.0.status.map(Into::into)
    }
    async fn surface(&self) -> f32 {
        self.0.surface
    }
    async fn tenant_private_spaces(&self) -> Option<String> {
        self.0.tenant_private_spaces.clone()
    }
    async fn usage_type(&self) -> Option<PropertyHabitationUsageType> {
        self.0.usage_type.map(Into::into)
    }
    async fn water_heating_method(&self) -> Option<PropertyUsageType> {
        self.0.water_heating_method.map(Into::into)
    }
    async fn id(&self) -> ID {
        self.0.id.into()
    }
    async fn lender_id(&self) -> Option<ID> {
        Some(self.0.lender_id.into())
    }
    //
    async fn expected_rents(&self) -> Option<Decimal> {
        None
    }
    async fn collected_rents(&self) -> Option<Decimal> {
        None
    }
    async fn lender(&self, ctx: &Context<'_>) -> Result<Option<Lender>> {
        let db_pool = ctx.data::<DbPool>()?;
        Ok(Some(
            db(db_pool.clone())
                .lenders()
                .by_id(&self.0.lender_id)?
                .lender()
                .into(),
        ))
    }
    async fn leases(&self) -> Option<Vec<Lease>> {
        Some(Vec::new())
    }
}

impl From<piteo::Property> for Property {
    fn from(item: piteo::Property) -> Self {
        Self(item)
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
    notice_id: Option<ID>,
    //
    lease: Option<Lease>,
    delay: Option<i32>,
    transactions: Option<Vec<Transaction>>,
}

impl From<piteo::Rent> for Rent {
    fn from(item: piteo::Rent) -> Self {
        Self {
            id: item.id.into(),
            period_end: item.period_end.into(),
            period_start: item.period_start.into(),
            amount: item.amount.into(),
            charges_amount: item.charges_amount.map(Into::into),
            full_amount: item.full_amount.into(),
            status: item.status,
            lease_id: item.lease_id.into(),
            receipt_id: item.receipt_id.map(Into::into),
            notice_id: item.notice_id.map(Into::into),
            lease: None,
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

impl From<piteo::Summary> for Summary {
    fn from(item: piteo::Summary) -> Self {
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
    birthdate: DateTime,
    birthplace: Option<String>,
    email: Email,
    first_name: String,
    last_name: String,
    display_name: String,
    note: Option<String>,
    phone_number: Option<PhoneNumber>,
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

impl From<piteo::Tenant> for Tenant {
    fn from(item: piteo::Tenant) -> Self {
        Self {
            display_name: item.display_name(),
            full_name: item.full_name(),
            account_id: item.account_id.into(),
            apl: Some(item.apl),
            birthdate: item.birthdate.into(),
            birthplace: item.birthplace,
            email: item.email.into(),
            first_name: item.first_name,
            last_name: item.last_name,
            note: item.note,
            phone_number: item.phone_number.map(Into::into),
            id: item.id.into(),
            lease_id: item.lease_id.map(Into::into),
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

#[derive(async_graphql::SimpleObject)]
pub struct Event {
    id: ID,
    created_at: DateTime,
    eventable_id: ID,
    eventable_type: String,
    r#type: EventType,
    object: Eventable,
}
