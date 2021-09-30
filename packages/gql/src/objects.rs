use crate::query::map_res;
use crate::unions::Eventable;
use crate::unions::LegalIdentity;
use crate::unions::WarrantIdentity;
use async_graphql::Result;
use async_graphql::ID;
use async_graphql::*;
use piteo::db;
use piteo::AccountStatus;
use piteo::Amount;
use piteo::AuthId;
use piteo::CandidacyStatus;
use piteo::Date;
use piteo::DateTime;
use piteo::Db;
use piteo::Email;
use piteo::EventType;
use piteo::EventableType;
use piteo::FileStatus;
use piteo::FileType;
use piteo::FurnishedLeaseDuration;
use piteo::LeaseRentPeriodicity;
use piteo::LeaseRentReferenceIrl;
use piteo::LeaseStatus;
use piteo::LeaseType;
use piteo::LegalEntityType;
use piteo::LenderFlexibility;
use piteo::Name;
use piteo::PersonRole;
use piteo::PhoneNumber;
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
use piteo::TenantStatus;
use piteo::TransactionType;
use piteo::Url;
use piteo::WarrantType;
use std::convert::TryInto;

// # Objects. https://async-graphql.github.io/async-graphql/en/define_complex_object.html

#[derive(async_graphql::SimpleObject)]
pub struct Error {
    message: String,
}

impl From<piteo::Error> for Error {
    fn from(item: piteo::Error) -> Self {
        Self {
            message: item.to_string(),
        }
    }
}

#[derive(async_graphql::SimpleObject)]
#[graphql(complex)]
pub struct Account {
    plan_id: Option<ID>,
    status: AccountStatus,
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
            status: item.status,
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
    pub display_name: String,
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
            email: item.email,
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
    download_url: Option<Url>,
    external_id: Option<String>,
    filename: Option<String>,
    preview_url: Option<Url>,
    status: Option<FileStatus>,
    r#type: FileType,
    updated_at: Option<DateTime>,
    id: ID,
}

impl From<piteo::File> for File {
    fn from(item: piteo::File) -> Self {
        Self {
            created_at: item.created_at.map(Into::into),
            download_url: item.download_url.map(Into::into),
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
    amount_paid: Amount,
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
    async fn deposit_amount(&self) -> Option<Amount> {
        self.0.deposit_amount.map(Into::into)
    }
    async fn effect_date(&self) -> DateTime {
        self.0.effect_date
    }
    async fn duration(&self) -> FurnishedLeaseDuration {
        self.0.duration
    }
    async fn signature_date(&self) -> Option<DateTime> {
        self.0.signature_date.map(Into::into)
    }
    async fn rent_amount(&self) -> Amount {
        self.0.rent_amount
    }
    async fn rent_charges_amount(&self) -> Option<Amount> {
        self.0.rent_charges_amount.map(Into::into)
    }
    async fn rent_full_amount(&self) -> Amount {
        self.0.rent_full_amount()
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
        Ok(db(&ctx.into())
            .rents()
            .by_lease_id(&self.0.id)?
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>())
    }
    async fn lease(&self) -> Option<File> {
        None
    }
    async fn tenants(&self, ctx: &Context<'_>) -> Result<Vec<Tenant>> {
        Ok(db(&ctx.into())
            .tenants()
            .by_lease_id(&self.0.id)?
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>())
    }
    async fn account(&self) -> Option<Account> {
        None
    }
    async fn property(&self, ctx: &Context<'_>) -> Result<Property> {
        Ok(db(&ctx.into())
            .properties()
            .by_id(&self.0.property_id)?
            .into())
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
    colocation_insurance_monthly_amount: Option<Amount>,
    colocation_insurance_total_amount: Option<Amount>,
    duration: Option<FurnishedLeaseDuration>,
    lender_fee_cap: Option<Amount>,
    lender_fee_cap_other: Option<String>,
    lender_fee_cap_prestations: Option<Amount>,
    other_conditions: Option<String>,
    rent_complement: Option<Amount>,
    rent_complement_property_justification: Option<String>,
    rent_first_amount: Option<Amount>,
    rent_irl: Option<LeaseRentReferenceIrl>,
    rent_irl_revision_date: Option<DateTime>,
    rent_maj_decree_increased_amount: Option<Amount>,
    rent_maj_decree_reference_amount: Option<Amount>,
    rent_majoration_decree: Option<bool>,
    rent_max_evolution_relocation: Option<bool>,
    rent_payment_date: Option<DateTime>,
    rent_payment_method: Option<RentPaymentMethod>,
    rent_payment_place: Option<String>,
    rent_periodicity: Option<LeaseRentPeriodicity>,
    rent_underestimated_method: Option<String>,
    rent_underestimated_monthly_variation: Option<Amount>,
    resolutary_clause: Option<String>,
    solidarity_clause: Option<String>,
    tenant_fee_cap_new_rental: Option<Amount>,
    tenant_fee_cap_prestations: Option<Amount>,
    tenant_fee_cap_report_by_meter: Option<Amount>,
    tenant_fee_cap_report_prestations: Option<Amount>,
    tenant_last_rent_amount: Option<Amount>,
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

pub struct Lender(piteo::Lender, Option<piteo::LegalIdentity>);

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
        match &self.1 {
            Some(legal_identity) => Ok(legal_identity.display_name()),
            None => Ok(db(&ctx.into())
                .lenders()
                .by_id(&self.0.id)?
                .1
                .display_name()),
        }
    }
    async fn identity(&self, ctx: &Context<'_>) -> Result<LegalIdentity> {
        match &self.1 {
            Some(legal_identity) => Ok(legal_identity.clone().into()),
            None => Ok(db(&ctx.into()).lenders().by_id(&self.0.id)?.1.into()),
        }
    }
}

impl From<piteo::Lender> for Lender {
    fn from(item: piteo::Lender) -> Self {
        Self(item, None)
    }
}

impl From<piteo::LenderWithIdentity> for Lender {
    fn from(item: piteo::LenderWithIdentity) -> Self {
        Self(item.0, Some(item.1))
    }
}

#[derive(async_graphql::SimpleObject)]
#[graphql(complex)]
#[graphql(name = "User")]
pub struct Person {
    auth_id: Option<AuthId>,
    email: Email,
    pub first_name: String,
    pub last_name: String,
    address: Option<Address>,
    #[graphql(name = "photoURL")]
    photo_url: Option<Url>,
    role: PersonRole,
    pub id: ID,
    phone_number: Option<PhoneNumber>,
    account_id: ID,
    pub display_name: String,
    //
    accounts: Option<Vec<Account>>,
}

#[async_graphql::ComplexObject]
impl Person {
    async fn account(&self, ctx: &Context<'_>) -> Result<Option<Account>> {
        let account_id = self.account_id.clone().try_into()?;

        Ok(Some(
            db(&ctx.into())
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
            auth_id: item.auth_id,
            email: item.email,
            first_name: item.first_name,
            last_name: item.last_name,
            address: item.address.map(Into::into),
            photo_url: item.photo_url,
            role: item.role,
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
    price: Option<Amount>,
    subtitle: Option<String>,
    title: Option<String>,
    id: ID,
    features: Vec<Feature>,
}

#[derive(async_graphql::SimpleObject)]
pub struct Advertisement {
    id: ID,
    created_at: Option<DateTime>,
    updated_at: Option<DateTime>,
    published: bool,
    lease_type: LeaseType,
    rent_amount: Amount,
    rent_charges_amount: Option<Amount>,
    deposit_amount: Option<Amount>,
    effect_date: DateTime,
    flexibility: Option<LenderFlexibility>,
    referral_lease_id: Option<ID>,
    property_id: ID,
}

impl From<piteo::Advertisement> for Advertisement {
    fn from(item: piteo::Advertisement) -> Self {
        Self {
            id: item.id.into(),
            created_at: item.created_at,
            updated_at: item.updated_at,
            published: item.published,
            lease_type: item.lease_type,
            rent_amount: item.rent_amount,
            rent_charges_amount: item.rent_charges_amount,
            deposit_amount: item.deposit_amount,
            effect_date: item.effect_date,
            flexibility: item.flexibility,
            referral_lease_id: item.referral_lease_id.map(Into::into),
            property_id: item.property_id.into(),
        }
    }
}

#[derive(async_graphql::SimpleObject)]
#[graphql(complex)]
pub struct Candidacy {
    id: ID,
    created_at: Option<DateTime>,
    updated_at: Option<DateTime>,
    status: CandidacyStatus,
    advertisement_id: ID,
    tenant_id: ID,
    move_in_date: DateTime,
    description: String,
}

#[async_graphql::ComplexObject]
impl Candidacy {
    async fn tenant(&self, ctx: &Context<'_>) -> Result<Tenant> {
        Ok(db(&ctx.into())
            .tenants()
            .by_id(&self.tenant_id.clone().try_into()?)
            .map(Into::into)?)
    }
}

impl From<piteo::Candidacy> for Candidacy {
    fn from(item: piteo::Candidacy) -> Self {
        Self {
            id: item.id.into(),
            created_at: item.created_at,
            updated_at: item.updated_at,
            status: item.status,
            advertisement_id: item.advertisement_id.into(),
            tenant_id: item.tenant_id.into(),
            move_in_date: item.move_in_date,
            description: item.description,
        }
    }
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
    async fn tax(&self) -> Option<Amount> {
        self.0.tax.map(Into::into)
    }
    async fn room_count(&self) -> PropertyRoomType {
        self.0.room_count
    }
    async fn status(&self) -> PropertyStatus {
        self.0.status
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
    async fn expected_rents(&self) -> Option<Amount> {
        None
    }
    async fn collected_rents(&self) -> Option<Amount> {
        None
    }
    async fn lender(&self, ctx: &Context<'_>) -> Result<Option<Lender>> {
        Ok(Some(
            db(&ctx.into()).lenders().by_id(&self.0.lender_id)?.0.into(),
        ))
    }
    async fn leases(&self, ctx: &Context<'_>) -> Result<Option<Vec<Lease>>> {
        Ok(Some(
            db(&ctx.into())
                .leases()
                .by_property_id(&self.0.id)?
                .into_iter()
                .map(Into::into)
                .collect::<Vec<_>>(),
        ))
    }
    async fn advertisements(&self, ctx: &Context<'_>) -> Result<Option<Vec<Advertisement>>> {
        Ok(Some(
            db(&ctx.into())
                .advertisements()
                .by_property_id(&self.0.id)
                .and_then(map_res)?,
        ))
    }
}

impl From<piteo::Property> for Property {
    fn from(item: piteo::Property) -> Self {
        Self(item)
    }
}

#[derive(async_graphql::SimpleObject)]
#[graphql(complex)]
pub struct Rent {
    id: ID,
    period_end: DateTime,
    period_start: DateTime,
    amount: Amount,
    charges_amount: Option<Amount>,
    full_amount: Amount,
    status: RentStatus,
    lease_id: ID,
    receipt_id: Option<ID>,
    notice_id: Option<ID>,
    //
    delay: Option<i32>,
    transactions: Option<Vec<Payment>>,
    receipt: Option<File>,
}

#[async_graphql::ComplexObject]
impl Rent {
    async fn lease(&self, ctx: &Context<'_>) -> Result<Lease> {
        Ok(db(&ctx.into())
            .leases()
            .by_id(&self.lease_id.clone().try_into()?)?
            .into())
    }
}

impl From<piteo::Rent> for Rent {
    fn from(item: piteo::Rent) -> Self {
        Self {
            id: item.id.into(),
            period_end: item.period_end,
            period_start: item.period_start,
            amount: item.amount,
            charges_amount: item.charges_amount.map(Into::into),
            full_amount: item.full_amount,
            status: item.status.into(),
            lease_id: item.lease_id.into(),
            receipt_id: item.receipt_id.map(Into::into),
            notice_id: item.notice_id.map(Into::into),
            delay: None,
            transactions: None,
            receipt: None,
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Summary {
    account_id: ID,
    created_at: DateTime,

    amount_expected: Amount,
    amount_received: Amount,
    amount_settled: Amount,
    amount_partial: Amount,
    amount_pending: Amount,

    n_expected: i32,
    n_received: i32,
    n_settled: i32,
    n_partial: i32,
    n_pending: i32,

    ratio_expected: f64,
    ratio_received: f64,
    ratio_settled: f64,
    ratio_partial: f64,
    ratio_pending: f64,

    variation_expected: f64,
    variation_received: f64,
    variation_settled: f64,
    variation_partial: f64,
    variation_pending: f64,

    payment_rate: f64,
    occupation_rate: f64,
}

impl From<piteo::Summary> for Summary {
    fn from(item: piteo::Summary) -> Self {
        Summary {
            account_id: item.account_id.into(),
            created_at: item.created_at,
            amount_expected: item.amount_expected,
            amount_received: item.amount_received,
            amount_settled: item.amount_settled,
            amount_partial: item.amount_partial,
            amount_pending: item.amount_pending,
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
#[graphql(complex)]
pub struct Tenant {
    account_id: ID,
    apl: Option<bool>,
    birthdate: Date,
    birthplace: Option<String>,
    email: Email,
    pub first_name: String,
    pub last_name: String,
    pub display_name: String,
    note: Option<String>,
    phone_number: Option<PhoneNumber>,
    pub id: ID,
    lease_id: Option<ID>,
    is_student: Option<bool>,
    //
    account: Option<Account>,
    property: Option<Property>,
    status: Option<TenantStatus>,
    full_name: String,
    short_name: Option<String>,
    last_transaction: Option<Payment>,
    property_name: Option<String>,
    rent_payed_this_year: Option<String>,
    unpaid_rent_amount: Option<u32>,
    files: Option<Vec<File>>,
    lease: Option<Lease>,
}

#[async_graphql::ComplexObject]
impl Tenant {
    async fn warrants(&self, ctx: &Context<'_>) -> Result<Option<Vec<Warrant>>> {
        Ok(Some(
            db(&ctx.into())
                .warrants()
                .by_tenant_id(&self.id.clone().try_into()?)
                .and_then(map_res)?,
        ))
    }
}

impl From<piteo::Tenant> for Tenant {
    fn from(item: piteo::Tenant) -> Self {
        Self {
            display_name: item.display_name(),
            full_name: item.full_name(),
            account_id: item.account_id.into(),
            apl: item.apl,
            birthdate: item.birthdate,
            birthplace: item.birthplace,
            email: item.email,
            first_name: item.first_name,
            last_name: item.last_name,
            note: item.note,
            phone_number: item.phone_number.map(Into::into),
            id: item.id.into(),
            lease_id: item.lease_id.map(Into::into),
            is_student: item.is_student,
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
pub struct Warrant {
    id: ID,
    r#type: WarrantType,
    identity: WarrantIdentity,
}

impl From<piteo::WarrantWithIdentity> for Warrant {
    fn from(item: piteo::WarrantWithIdentity) -> Self {
        Self {
            id: item.0.id.into(),
            r#type: item.0.type_,
            identity: item.1.into(),
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct ProfessionalWarrant {
    name: String,
    identifier: String,
}

impl From<piteo::ProfessionalWarrant> for ProfessionalWarrant {
    fn from(item: piteo::ProfessionalWarrant) -> Self {
        Self {
            name: item.name,
            identifier: item.identifier,
        }
    }
}

#[derive(async_graphql::SimpleObject)]
#[graphql(complex)]
#[graphql(name = "Transaction")]
pub struct Payment {
    id: ID,
    date: DateTime,
    amount: Amount,
    r#type: TransactionType,
    rent_id: ID,
    //
    account_id: Option<ID>,
}

#[async_graphql::ComplexObject]
impl Payment {
    async fn lease(&self, ctx: &Context<'_>) -> Result<Lease> {
        Ok(db(&ctx.into())
            .leases()
            .by_rent_id(&self.rent_id.clone().try_into()?)?
            .into())
    }
}

impl From<piteo::Payment> for Payment {
    fn from(item: piteo::Payment) -> Self {
        Self {
            id: item.id.into(),
            date: item.date,
            amount: item.amount,
            r#type: item.type_,
            rent_id: item.rent_id.into(),
            account_id: None,
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Event {
    id: ID,
    created_at: Option<DateTime>,
    updated_at: Option<DateTime>,
    eventable_id: ID,
    #[graphql(name = "eventableType")]
    eventable_type: EventableType,
    r#type: EventType,
    #[graphql(name = "object")]
    eventable: Eventable,
}

impl From<piteo::EventWithEventable> for Event {
    fn from(item: piteo::EventWithEventable) -> Self {
        Self {
            id: item.0.id.into(),
            created_at: item.0.created_at,
            updated_at: item.0.updated_at,
            eventable_id: item.0.eventable_id.into(),
            eventable_type: item.0.eventable_type,
            r#type: item.0.type_,
            eventable: item.1.into(),
        }
    }
}

// # Enums

#[derive(Copy, Clone, Debug, PartialEq, Eq, Enum)]
enum RentStatus {
    Pending,
    Settled,
    Partial,
}

impl From<piteo::RentStatus> for RentStatus {
    fn from(item: piteo::RentStatus) -> Self {
        match item {
            piteo::RentStatus::Open => Self::Pending,
            piteo::RentStatus::Paid => Self::Settled,
            piteo::RentStatus::PartiallyPaid => Self::Partial,
        }
    }
}
