use crate::auth::AddressInput;
use crate::database::Db;
use crate::AuthId;
use async_graphql::InputObject;
use eyre::Error;
use piteo_data::Amount;
use piteo_data::Property;
use piteo_data::PropertyBuildPeriodType;
use piteo_data::PropertyBuildingLegalStatus;
use piteo_data::PropertyData;
use piteo_data::PropertyEnergyClass;
use piteo_data::PropertyGasEmission;
use piteo_data::PropertyHabitationUsageType;
use piteo_data::PropertyId;
use piteo_data::PropertyRoomType;
use piteo_data::PropertyStatus;
use piteo_data::PropertyUsageType;
use validator::Validate;

// # Input

#[derive(InputObject, Validate)]
#[graphql(name = "PropertyUpdateInput")]
pub struct UpdatePropertyInput {
    pub address: Option<AddressInput>,
    pub build_period: Option<PropertyBuildPeriodType>,
    pub building_legal_status: Option<PropertyBuildingLegalStatus>,
    pub common_spaces: Option<String>,
    pub energy_class: Option<PropertyEnergyClass>,
    pub equipments: Option<String>,
    pub gas_emission: Option<PropertyGasEmission>,
    pub heating_method: Option<PropertyUsageType>,
    pub housing_type: Option<PropertyUsageType>,
    pub id: PropertyId,
    pub name: Option<String>,
    pub note: Option<String>,
    pub ntic_equipments: Option<String>,
    pub other_spaces: Option<String>,
    pub room_count: Option<PropertyRoomType>,
    pub status: Option<PropertyStatus>,
    pub surface: Option<f64>,
    pub tax: Option<Amount>,
    pub tenant_private_spaces: Option<String>,
    pub usage_type: Option<PropertyHabitationUsageType>,
    pub water_heating_method: Option<PropertyUsageType>,
}

// # Operation

pub fn update_property(
    db: impl Db,
    _auth_id: AuthId,
    input: UpdatePropertyInput,
) -> Result<Property, Error> {
    input.validate()?;

    db.properties().update(input.into())
}

// # Impls

impl From<UpdatePropertyInput> for PropertyData {
    fn from(item: UpdatePropertyInput) -> Self {
        Self {
            id: item.id,
            account_id: Default::default(),
            address: item.address.map(Into::into),
            build_period: item.build_period,
            building_legal_status: item.building_legal_status,
            common_spaces: item.common_spaces,
            energy_class: item.energy_class,
            equipments: item.equipments,
            gas_emission: item.gas_emission,
            heating_method: item.heating_method,
            housing_type: item.housing_type,
            lender_id: Default::default(),
            name: item.name,
            note: item.note,
            ntic_equipments: item.ntic_equipments,
            other_spaces: item.other_spaces,
            room_count: item.room_count,
            status: item.status,
            surface: item.surface,
            tax: item.tax,
            tenant_private_spaces: item.tenant_private_spaces,
            usage_type: item.usage_type,
            water_heating_method: item.water_heating_method,
        }
    }
}
