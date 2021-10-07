use crate::auth::AddressInput;
use crate::error::Result;
use crate::AuthId;
use async_graphql::InputObject;
use piteo_core::database::Db;
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
    pub description: Option<String>,
    pub ntic_equipments: Option<String>,
    pub other_spaces: Option<String>,
    pub room_count: Option<PropertyRoomType>,
    pub status: Option<PropertyStatus>,
    pub surface: Option<f32>,
    pub tax: Option<Amount>,
    pub tenant_private_spaces: Option<String>,
    pub usage_type: Option<PropertyHabitationUsageType>,
    pub water_heating_method: Option<PropertyUsageType>,
}

// # Operation

pub fn update_property(
    db: &impl Db,
    _auth_id: &AuthId,
    input: UpdatePropertyInput,
) -> Result<Property> {
    input.validate()?;

    db.properties().update(PropertyData {
        id: input.id,
        account_id: Default::default(),
        address: input.address.map(Into::into),
        build_period: input.build_period,
        building_legal_status: input.building_legal_status,
        common_spaces: input.common_spaces,
        energy_class: input.energy_class,
        equipments: input.equipments,
        gas_emission: input.gas_emission,
        heating_method: input.heating_method,
        housing_type: input.housing_type,
        lender_id: Default::default(),
        name: input.name,
        note: input.note,
        ntic_equipments: input.ntic_equipments,
        other_spaces: input.other_spaces,
        room_count: input.room_count,
        status: input.status,
        surface: input.surface,
        tax: input.tax,
        tenant_private_spaces: input.tenant_private_spaces,
        usage_type: input.usage_type,
        water_heating_method: input.water_heating_method,
    })
}
