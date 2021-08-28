use crate::enums::LeaseType;
use crate::enums::RentStatus;
use crate::scalars::AuthId;
use crate::scalars::DateTime;
use crate::scalars::Decimal;
use async_graphql::ID;

// # Objects. https://async-graphql.github.io/async-graphql/en/define_simple_object.html

#[derive(async_graphql::SimpleObject)]
pub struct Address {
    city: String,
    line1: String,
    line2: Option<String>,
    postal_code: String,
}

impl From<piteo_core::Address> for Address {
    fn from(item: piteo_core::Address) -> Self {
        Self {
            city: item.city.unwrap_or_default(),
            line1: item.line1.unwrap_or_default(),
            line2: item.line2,
            postal_code: item.postal_code.unwrap_or_default(),
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Lease {
    account_id: ID,
    deposit_amount: Option<Decimal>,
    effect_date: DateTime,
    signature_date: Option<DateTime>,
    rent_amount: Decimal,
    rent_charges_amount: Option<Decimal>,
    r#type: LeaseType,
    lease_id: Option<ID>,
    property_id: ID,
    id: ID,
    data: Option<LeaseData>,
    expired_at: Option<DateTime>,
    renew_date: Option<DateTime>,
}

impl From<piteo_core::Lease> for Lease {
    fn from(item: piteo_core::Lease) -> Self {
        Self {
            account_id: item.account_id.into(),
            deposit_amount: item.deposit_amount.map(Decimal::from),
            effect_date: item.effect_date.into(),
            signature_date: item.signature_date.map(DateTime::from),
            rent_amount: item.rent_amount.into(),
            rent_charges_amount: item.rent_charges_amount.map(Decimal::from),
            r#type: item.r#type.into(),
            lease_id: item.lease_id.map(ID::from),
            property_id: item.property_id.into(),
            id: item.id.into(),
            data: item.data.map(LeaseData::from),
            expired_at: item.expired_at.map(DateTime::from),
            renew_date: item.renew_date.map(DateTime::from),
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct LeaseData {
    duration: Option<String>,
    rent_payment_method: Option<String>,
}

impl From<piteo_core::LeaseData> for LeaseData {
    fn from(item: piteo_core::LeaseData) -> Self {
        Self {
            duration: item.duration,
            rent_payment_method: item.rent_payment_method,
        }
    }
}

#[derive(async_graphql::SimpleObject)]
pub struct Person {
    auth_id: AuthId,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
    photo_url: Option<String>,
    role: Option<String>,
    id: ID,
    phone_number: Option<String>,
    account_id: Option<ID>,
}

impl From<piteo_core::Person> for Person {
    fn from(item: piteo_core::Person) -> Self {
        Self {
            auth_id: item.auth_id.into(),
            email: item.email,
            first_name: item.first_name,
            last_name: item.last_name,
            photo_url: item.photo_url,
            role: item.role,
            id: item.id.into(),
            phone_number: item.phone_number,
            account_id: item.account_id.map(ID::from),
        }
    }
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
    receipt_id: ID,
    transaction_id: ID,
    notice_id: Option<ID>,
}

impl From<piteo_core::Rent> for Rent {
    fn from(_item: piteo_core::Rent) -> Self {
        todo!()
    }
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
    note: Option<String>,
    phone_number: Option<String>,
    role: Option<String>,
    id: ID,
    lease_id: Option<ID>,
    visale_id: Option<String>,
}

impl From<piteo_core::Tenant> for Tenant {
    fn from(item: piteo_core::Tenant) -> Self {
        Self {
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
        }
    }
}
