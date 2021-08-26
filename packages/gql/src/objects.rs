// # Objects

#[derive(async_graphql::SimpleObject)]
pub struct Address {
    city: String,
    line1: String,
    line2: Option<String>,
    postal_code: String,
}

#[derive(async_graphql::SimpleObject)]
pub struct Person {
    auth_id: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
    photo_url: Option<String>,
    role: Option<String>,
    id: String,
    phone_number: Option<String>,
    account_id: Option<String>,
}

#[derive(async_graphql::SimpleObject)]
pub struct Property {
    account_id: Option<String>,
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
    id: String,
    lender_id: String,
}

// # Impls

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

impl From<piteo_core::Person> for Person {
    fn from(item: piteo_core::Person) -> Self {
        Self {
            auth_id: item.auth_id.inner().to_string(),
            email: item.email,
            first_name: item.first_name,
            last_name: item.last_name,
            photo_url: item.photo_url,
            role: item.role,
            id: item.id.to_string(),
            phone_number: item.phone_number,
            account_id: item.account_id.map(|id| id.to_string()),
        }
    }
}

impl From<piteo_core::Property> for Property {
    fn from(item: piteo_core::Property) -> Self {
        Self {
            account_id: item.account_id.map(|id| id.to_string()),
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
            id: item.id.to_string(),
            lender_id: item.lender_id.to_string(),
        }
    }
}
