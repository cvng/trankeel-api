use crate::enums::LeaseStatus;
use crate::enums::LeaseType;
use crate::enums::RentStatus;
use crate::enums::TenantStatus;
use crate::scalars::AuthId;
use crate::scalars::DateTime;
use crate::scalars::Decimal;
use crate::scalars::Email;
use crate::scalars::PhoneNumber;
use crate::unions::Identity;
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
    status: Option<String>,
    stripe_customer_id: Option<String>,
    stripe_subscription_id: Option<String>,
    trial_end: Option<DateTime>,
    owner_id: String,
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
            status: item.status,
            stripe_customer_id: item.stripe_customer_id,
            stripe_subscription_id: item.stripe_subscription_id,
            trial_end: item.trial_end.map(DateTime::from),
            owner_id: item.owner_id,
            id: item.id.into(),
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Address {
    city: String,
    country: String,
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
            country: item.country.unwrap_or_default(),
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
    legal_entity_identifier: String,
    legal_entity_type: String, // LegalEntityType
    legal_entity_type_other: String,
    phone_number: PhoneNumber,
}

#[derive(async_graphql::SimpleObject)]
pub struct Feature {
    available: bool,
    title: String,
    key: String,
}

#[derive(async_graphql::SimpleObject)]
pub struct File {
    created_at: DateTime,
    download_url: Option<String>,
    external_id: Option<String>,
    filename: Option<String>,
    preview_url: Option<String>,
    status: Option<String>,
    r#type: String,
    updated_at: Option<DateTime>,
    id: ID,
}

#[derive(async_graphql::SimpleObject)]
pub struct Invoice {
    id: ID,
    number: usize,
    amount_paid: Decimal,
    invoice_pdf: String,
    period_end: DateTime,
    status: String,
    plan_code: String,
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
    rents: Vec<Rent>,
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
            rents: Vec::new(),
            tenants: Vec::new(),
            lease: None,
            account: None,
            property: None,
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct LeaseFurnishedData {
    duration: Option<String>,
    rent_payment_method: Option<String>,
}

impl From<piteo_core::LeaseData> for LeaseFurnishedData {
    fn from(item: piteo_core::LeaseData) -> Self {
        Self {
            duration: item.duration,
            rent_payment_method: item.rent_payment_method,
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Lender {
    id: ID,
    account_id: ID,
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
            account_id: item.account_id.into(),
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
    auth_id: AuthId,
    email: Email,
    first_name: Option<String>,
    last_name: Option<String>,
    address: Option<Address>,
    #[graphql(name = "photoURL")]
    photo_url: Option<String>,
    role: Option<String>,
    id: ID,
    phone_number: Option<String>,
    account_id: ID,
    display_name: String,
}

#[async_graphql::ComplexObject]
impl Person {
    async fn account(&self, ctx: &Context<'_>) -> Result<Account> {
        let conn = ctx.data::<DbPool>()?.get()?;
        let account_id = self.account_id.clone().try_into()?;

        Ok(auth::find_by_id(&conn, account_id).map(Account::from)?)
    }
}

impl From<piteo_core::Person> for Person {
    fn from(item: piteo_core::Person) -> Self {
        Self {
            display_name: item.display_name(),
            auth_id: item.auth_id.into(),
            email: item.email.into(),
            first_name: item.first_name,
            last_name: item.last_name,
            address: item.address.map(Address::from),
            photo_url: item.photo_url,
            role: item.role,
            id: item.id.into(),
            phone_number: item.phone_number,
            account_id: item.account_id.unwrap_or_default().into(),
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Plan {
    code: String,
    price: Option<Decimal>,
    subtitle: Option<String>,
    title: Option<String>,
    id: ID,
    features: Vec<Feature>,
}

#[derive(async_graphql::SimpleObject)]
pub struct Property {
    account_id: Option<ID>,
    address: Address,
    build_period: Option<String>,
    building_legal_status: Option<String>,
    common_spaces: Option<String>,
    energy_class: Option<String>,
    equipments: Option<String>,
    gas_emission: Option<String>,
    heating_method: Option<String>,
    housing_type: Option<String>,
    name: String,
    note: Option<String>,
    ntic_equipments: Option<String>,
    other_spaces: Option<String>,
    tax: Option<f64>,
    room_count: String,
    status: Option<String>,
    surface: i32,
    tenant_private_spaces: Option<String>,
    usage_type: Option<String>,
    water_heating_method: Option<String>,
    id: ID,
    lender_id: ID,
    //
    expected_rents: Option<Decimal>,
    collected_rents: Option<Decimal>,
    lender: Option<Lender>,
    leases: Vec<Lease>,
}

impl From<piteo_core::Property> for Property {
    fn from(item: piteo_core::Property) -> Self {
        Self {
            account_id: item.account_id.map(ID::from),
            address: item.address.into(),
            build_period: item.build_period,
            building_legal_status: item.building_legal_status,
            common_spaces: item.common_spaces,
            energy_class: item.energy_class,
            equipments: item.equipments,
            gas_emission: item.gas_emission,
            heating_method: item.heating_method,
            housing_type: item.housing_type,
            name: item.name,
            note: item.note,
            ntic_equipments: item.ntic_equipments,
            other_spaces: item.other_spaces,
            tax: item.tax,
            room_count: item.room_count,
            status: item.status,
            surface: item.surface,
            tenant_private_spaces: item.tenant_private_spaces,
            usage_type: item.usage_type,
            water_heating_method: item.water_heating_method,
            id: item.id.into(),
            lender_id: item.lender_id.into(),
            expected_rents: None,
            collected_rents: None,
            lender: None,
            leases: Vec::new(),
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
    account_id: ID,
    lease_id: ID,
    receipt_id: Option<ID>,
    transaction_id: Option<ID>,
    notice_id: Option<ID>,
    //
    transactions: Vec<Transaction>,
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
            account_id: item.account_id.into(),
            lease_id: item.lease_id.into(),
            receipt_id: item.receipt_id.map(ID::from),
            transaction_id: item.transaction_id.map(ID::from),
            notice_id: item.notice_id.map(ID::from),
            transactions: Vec::new(),
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
    n_expected: usize,
    n_received: usize,
    n_settled: usize,
    n_partial: usize,
    n_pending: usize,
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
    progress: usize,
}

#[derive(async_graphql::SimpleObject)]
pub struct Tenant {
    account_id: ID,
    apl: bool,
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
    status: TenantStatus,
    full_name: String,
    short_name: String,
    last_transaction: Option<Transaction>,
    property_name: Option<String>,
    rent_payed_this_year: Option<String>,
    unpaid_rent_amount: Option<String>,
    files: Vec<File>,
    lease: Option<Lease>,
}

impl From<piteo_core::Tenant> for Tenant {
    fn from(item: piteo_core::Tenant) -> Self {
        Self {
            display_name: item.display_name(),
            full_name: item.full_name(),
            account_id: item.account_id.into(),
            apl: item.apl,
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
            status: TenantStatus::New,
            short_name: String::new(),
            last_transaction: None,
            property_name: None,
            rent_payed_this_year: None,
            unpaid_rent_amount: None,
            files: Vec::new(),
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
    r#type: String,
    //
    lease: Option<Lease>,
}
